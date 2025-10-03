//! Template rendering engines
//!
//! Provides different template rendering implementations.

use crate::domain::chat::templates::core::{
    ChatTemplate as CandleChatTemplate, TemplateContext as CandleTemplateContext,
    TemplateError as CandleTemplateError, TemplateResult as CandleTemplateResult,
    TemplateValue as CandleTemplateValue,
};

/// Template rendering engine trait
pub trait TemplateEngine: Send + Sync {
    /// Render a template with the given context
    ///
    /// # Errors
    ///
    /// Returns `CandleTemplateError` if template rendering fails
    fn render(
        &self,
        template: &CandleChatTemplate,
        context: &CandleTemplateContext,
    ) -> CandleTemplateResult<String>;

    /// Check if the engine supports the template format
    fn supports(&self, template: &CandleChatTemplate) -> bool;

    /// Get engine name
    fn name(&self) -> &'static str;
}

/// Simple string interpolation engine
pub struct SimpleEngine;

impl TemplateEngine for SimpleEngine {
    fn render(
        &self,
        template: &CandleChatTemplate,
        context: &CandleTemplateContext,
    ) -> CandleTemplateResult<String> {
        let mut result = template.get_content().clone();

        // Simple variable replacement: {{variable_name}}
        for (name, value) in context.variables() {
            let placeholder = format!("{{{{{name}}}}}");
            let replacement = match value {
                CandleTemplateValue::String(s) => s.as_str(),
                CandleTemplateValue::Number(n) => &n.to_string(),
                CandleTemplateValue::Boolean(b) => {
                    if *b {
                        "true"
                    } else {
                        "false"
                    }
                }
                CandleTemplateValue::Array(_) => "[array]", // Simplified
                CandleTemplateValue::Object(_) => "[object]", // Simplified
                CandleTemplateValue::Null => "",
            };
            result = result.replace(&placeholder, replacement);
        }

        Ok(result)
    }

    fn supports(&self, _template: &CandleChatTemplate) -> bool {
        true // Simple engine supports all templates
    }

    fn name(&self) -> &'static str {
        "simple"
    }
}

/// Template engine registry
pub struct EngineRegistry {
    engines: Vec<Box<dyn TemplateEngine>>,
}

impl EngineRegistry {
    /// Create a new engine registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            engines: Vec::new(),
        }
    }

    /// Create a registry with default engines
    #[must_use]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(SimpleEngine));
        registry
    }

    /// Register a new engine
    pub fn register(&mut self, engine: Box<dyn TemplateEngine>) {
        self.engines.push(engine);
    }

    /// Find the best engine for a template
    pub fn find_engine(&self, template: &CandleChatTemplate) -> Option<&dyn TemplateEngine> {
        self.engines
            .iter()
            .find(|engine| engine.supports(template))
            .map(AsRef::as_ref)
    }

    /// Render a template using the best available engine
    ///
    /// # Errors
    ///
    /// Returns `CandleTemplateError` if:
    /// - No suitable engine is found for the template
    /// - Template rendering fails
    pub fn render(
        &self,
        template: &CandleChatTemplate,
        context: &CandleTemplateContext,
    ) -> CandleTemplateResult<String> {
        match self.find_engine(template) {
            Some(engine) => engine.render(template, context),
            None => Err(CandleTemplateError::RenderError {
                message: "No suitable rendering engine found".to_string(),
            }),
        }
    }
}

impl Default for EngineRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
