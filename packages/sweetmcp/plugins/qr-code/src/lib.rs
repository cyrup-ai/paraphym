use base64::Engine;
use extism_pdk::*;
use log::{debug, trace};
use qrcode_png::{Color, QrCode, QrCodeEcc};
use serde_json::Value;
use sweetmcp_plugin_builder::prelude::*;
use sweetmcp_plugin_builder::{CallToolResult, Ready};

/// QR code generation tool using plugin-builder
struct QrCodeTool;

impl McpTool for QrCodeTool {
    const NAME: &'static str = "qr-code";

    fn description(builder: DescriptionBuilder) -> DescriptionBuilder {
        builder
            .does("Generate QR codes as PNG images from text or data input")
            .when("you need to create scannable codes for URLs, WiFi credentials, or contact information")
            .when("you need to generate QR codes for mobile app deep links or authentication")
            .when("you need to encode data for easy sharing at events or on printed materials")
            .when("you need to create codes for digital business cards or marketing campaigns")
            .when("you want to bridge physical and digital experiences with scannable content")
            .perfect_for("mobile integration, contactless sharing, event management, and marketing materials")
            .operation("generate", "Create a QR code PNG image from input data with configurable error correction")
            .requires("Base64 encoding capability for image output")
            .not_for("very large data that exceeds QR code capacity limits")
            .always_for("creating shareable, scannable codes from text or structured data")
    }

    fn schema(builder: SchemaBuilder) -> Value {
        builder
            .required_string("data", "Text or data to encode in the QR code")
            .optional_string(
                "ecc",
                "Error correction level (1=low, 2=medium, 3=quartile, 4=high, default=4)",
            )
            .build()
    }

    fn execute(args: Value) -> Result<CallToolResult, Error> {
        let data = args
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::msg("data parameter required"))?;

        debug!("Generating QR code for {} bytes of data", data.len());

        let ecc_level = args
            .get("ecc")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(4);

        let ecc = to_ecc(ecc_level);
        debug!("Error correction level: {:?}", ecc);

        match generate_qr_code(data, ecc) {
            Ok(base64_data) => {
                debug!("QR code generation successful, encoded as base64");
                use sweetmcp_plugin_builder::{CallToolResult, Content, ContentType};
                Ok(CallToolResult {
                    is_error: None,
                    content: vec![Content {
                        annotations: None,
                        text: None,
                        mime_type: Some("image/png".into()),
                        r#type: ContentType::Image,
                        data: Some(base64_data),
                    }],
                })
            }
            Err(e) => Ok(ContentBuilder::error(&format!(
                "Failed to generate QR code: {}",
                e
            ))),
        }
    }
}

/// Generate QR code and return base64 encoded PNG
fn generate_qr_code(data: &str, ecc: QrCodeEcc) -> Result<String, Box<dyn std::error::Error>> {
    trace!("Encoding data into QR code structure");
    let mut code = QrCode::new(data, ecc)?;
    debug!("QR code matrix created");
    
    trace!("Setting QR code margin to 10 pixels");
    code.margin(10);
    trace!("Setting QR code zoom to 10x");
    code.zoom(10);

    trace!("Generating PNG image with grayscale colors");
    let png_bytes = code.generate(Color::Grayscale(0, 255))?;
    debug!("PNG generation complete: {} bytes", png_bytes.len());
    
    trace!("Encoding PNG to base64");
    let base64_data = base64::engine::general_purpose::STANDARD.encode(png_bytes);
    trace!("Base64 encoding complete: {} chars", base64_data.len());

    Ok(base64_data)
}

/// Convert numeric ECC level to QrCodeEcc enum
fn to_ecc(num: u8) -> QrCodeEcc {
    match num {
        1 => QrCodeEcc::Low,
        2 => QrCodeEcc::Medium,
        3 => QrCodeEcc::Quartile,
        4 | _ => QrCodeEcc::High,
    }
}

/// Create the plugin instance
#[allow(dead_code)]
fn plugin() -> McpPlugin<Ready> {
    mcp_plugin("qr-code")
        .description("High-quality QR code generator with configurable error correction")
        .tool::<QrCodeTool>()
        .serve()
}

// Generate standard MCP entry points
sweetmcp_plugin_builder::generate_mcp_functions!(plugin);
