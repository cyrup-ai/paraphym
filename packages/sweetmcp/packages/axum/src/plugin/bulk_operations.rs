use crate::db::Dao;
use crate::db::dao::entities::{PluginEntity, ToolEntity, PromptEntity};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use futures::StreamExt;

#[derive(Serialize, Deserialize)]
pub struct BulkExport {
    pub version: String,
    pub exported_at: DateTime<Utc>,
    pub plugins: Vec<PluginEntity>,
    pub tools: Vec<ToolEntity>,
    pub prompts: Vec<PromptEntity>,
}

pub async fn export_all() -> Result<BulkExport, String> {
    let client = crate::db::client::get_db_client()
        .map_err(|e| format!("Database unavailable: {}", e))?;
    
    let plugin_dao = Dao::<PluginEntity>::new((*client).clone());
    let tool_dao = Dao::<ToolEntity>::new((*client).clone());
    let prompt_dao = Dao::<PromptEntity>::new((*client).clone());
    
    // Collect all entities
    let plugins = {
        let stream = plugin_dao.find().await;
        futures::pin_mut!(stream);
        stream.collect().await
    };
    
    let tools = {
        let stream = tool_dao.find().await;
        futures::pin_mut!(stream);
        stream.collect().await
    };
    
    let prompts = {
        let stream = prompt_dao.find().await;
        futures::pin_mut!(stream);
        stream.collect().await
    };
    
    Ok(BulkExport {
        version: "1.0".to_string(),
        exported_at: Utc::now(),
        plugins,
        tools,
        prompts,
    })
}

pub async fn import_all(data: BulkExport) -> Result<ImportStats, String> {
    let client = crate::db::client::get_db_client()
        .map_err(|e| format!("Database unavailable: {}", e))?;
    
    let plugin_dao = Dao::<PluginEntity>::new((*client).clone());
    let tool_dao = Dao::<ToolEntity>::new((*client).clone());
    let prompt_dao = Dao::<PromptEntity>::new((*client).clone());
    
    let mut stats = ImportStats::default();
    
    // Import using create_batch
    let plugin_results = plugin_dao.create_batch(&mut data.plugins.clone()).await;
    for result in plugin_results {
        match result {
            Ok(_) => stats.plugins_imported += 1,
            Err(e) => stats.errors.push(format!("Plugin: {}", e)),
        }
    }
    
    let tool_results = tool_dao.create_batch(&mut data.tools.clone()).await;
    for result in tool_results {
        match result {
            Ok(_) => stats.tools_imported += 1,
            Err(e) => stats.errors.push(format!("Tool: {}", e)),
        }
    }
    
    let prompt_results = prompt_dao.create_batch(&mut data.prompts.clone()).await;
    for result in prompt_results {
        match result {
            Ok(_) => stats.prompts_imported += 1,
            Err(e) => stats.errors.push(format!("Prompt: {}", e)),
        }
    }
    
    Ok(stats)
}

#[derive(Default, Serialize)]
pub struct ImportStats {
    pub plugins_imported: usize,
    pub tools_imported: usize,
    pub prompts_imported: usize,
    pub errors: Vec<String>,
}
