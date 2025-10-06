use chrono::Utc;
use extism_pdk::*;
use log::{debug, trace};
use serde_json::{Value, json};
use sweetmcp_plugin_builder::prelude::*;
use sweetmcp_plugin_builder::{CallToolResult, Ready};

/// Time tool using plugin-builder
struct TimeTool;

impl McpTool for TimeTool {
    const NAME: &'static str = "time";

    fn description(builder: DescriptionBuilder) -> DescriptionBuilder {
        builder
            .does("Get current time in various formats and parse time strings")
            .when("you need to get the current UTC time")
            .when("you need to parse or format time strings")
            .when("you need to work with timestamps")
            .perfect_for("scheduling, logging, time-based calculations, and date/time operations")
    }

    fn schema(builder: SchemaBuilder) -> Value {
        builder
            .required_enum(
                "name",
                "Time operation to perform",
                &["get_time_utc", "parse_time"],
            )
            .optional_string(
                "time_string",
                "Time string to parse (for parse_time operation)",
            )
            .build()
    }

    fn execute(args: Value) -> Result<CallToolResult, Error> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::msg("name parameter required"))?;

        debug!("Executing time operation: {}", name);

        match name {
            "get_time_utc" => {
                debug!("Getting current UTC time");
                let now = Utc::now();
                trace!("Current UTC time: {}", now);
                
                let timestamp = now.timestamp().to_string();
                trace!("Timestamp (Unix epoch): {}", timestamp);
                
                let rfc2822 = now.to_rfc2822().to_string();
                trace!("RFC2822 format: {}", rfc2822);
                
                debug!("UTC time calculation complete");
                Ok(ContentBuilder::text(
                    json!({
                        "utc_time": timestamp,
                        "utc_time_rfc2822": rfc2822,
                    })
                    .to_string(),
                ))
            }
            "parse_time" => {
                let time_string = args
                    .get("time_string")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::msg("time_string parameter required for parse_time"))?;

                debug!("Parsing time string in RFC2822 format");
                trace!("Input time string: {}", time_string);

                match chrono::DateTime::parse_from_rfc2822(time_string) {
                    Ok(dt) => {
                        debug!("Time string parsed successfully");
                        trace!("Parsed datetime: {}", dt);
                        
                        let timestamp = dt.timestamp().to_string();
                        trace!("Converted to timestamp: {}", timestamp);
                        
                        let formatted = dt.to_rfc2822().to_string();
                        trace!("Formatted output: {}", formatted);
                        
                        Ok(ContentBuilder::text(
                            json!({
                                "parsed_time": timestamp,
                                "formatted": formatted,
                            })
                            .to_string(),
                        ))
                    }
                    Err(e) => {
                        debug!("Time parsing failed: {}", e);
                        Ok(ContentBuilder::error(format!(
                            "Failed to parse time: {}",
                            e
                        )))
                    }
                }
            }
            _ => {
                debug!("Unknown time operation requested: {}", name);
                Ok(ContentBuilder::error(format!(
                    "Unknown time operation: {}",
                    name
                )))
            }
        }
    }
}

/// Create the plugin instance
#[allow(dead_code)]
fn plugin() -> McpPlugin<Ready> {
    mcp_plugin("time")
        .description("Time operations including getting current time and parsing time strings")
        .tool::<TimeTool>()
        .serve()
}

// Generate standard MCP entry points
sweetmcp_plugin_builder::generate_mcp_functions!(plugin);
