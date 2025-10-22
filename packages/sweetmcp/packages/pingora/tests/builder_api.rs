use std::sync::Arc;

use sweetmcp::edge::core::service::EdgeServiceError;
use sweetmcp::edge::EdgeServiceBuilder;
use sweetmcp::peer_discovery::PeerRegistry;
use sweetmcp::rate_limit::{distributed::DistributedRateLimitManager, limiter::AdvancedRateLimitManager, RateLimiter};

#[test]
fn builder_status_and_helpers() {
    // Exercise status(), reset(), clone_builder(), with_preset(), BuilderStatus methods
    let builder = EdgeServiceBuilder::new();
    let status = builder.status();
    // Invoke BuilderStatus helpers to ensure they are used
    let _ = status.is_complete();
    let _ = status.completion_percentage();
    let _missing = status.missing_components();

    // Clone and reset
    let _clone = builder.clone_builder();
    let _reset = builder.reset();

    // Use BuilderPreset enum values via with_preset
    let _dev = EdgeServiceBuilder::new().with_preset(sweetmcp::edge::core::builder::BuilderPreset::Development);
    let _prod = EdgeServiceBuilder::new().with_preset(sweetmcp::edge::core::builder::BuilderPreset::Production);
    let _test = EdgeServiceBuilder::new().with_preset(sweetmcp::edge::core::builder::BuilderPreset::Testing);
}

#[tokio::test]
async fn builder_with_custom_rate_limiter_and_build_for_testing() -> Result<(), EdgeServiceError> {
    // Use custom Distributed limiter
    let rl1 = RateLimiter::Distributed(Arc::new(DistributedRateLimitManager::new()));
    let svc1 = EdgeServiceBuilder::new()
        .with_custom_rate_limiter(rl1)
        .build_for_testing()?;

    // Use custom Advanced limiter
    let rl2 = RateLimiter::Advanced(Arc::new(AdvancedRateLimitManager::new(10.0, 100, 60)));
    let svc2 = EdgeServiceBuilder::new()
        .with_custom_rate_limiter(rl2)
        .build_for_testing()?;

    // From service back to builder
    let _b1 = EdgeServiceBuilder::from_service(&svc1);
    let _b2 = EdgeServiceBuilder::from_service(&svc2);

    Ok(())
}

#[tokio::test]
async fn build_multiple_services() -> Result<(), EdgeServiceError> {
    use sweetmcp::circuit_breaker::{CircuitBreakerConfig, CircuitBreakerManager};

    // Base builder with essentials (bridge channel and peer registry)
    let (tx, _rx) = tokio::sync::mpsc::channel(10);
    let circuit_config = CircuitBreakerConfig {
        error_threshold_percentage: 50,
        request_volume_threshold: 20,
        sleep_window: std::time::Duration::from_secs(5),
        half_open_requests: 3,
        metrics_window: std::time::Duration::from_secs(10),
    };
    let cb_mgr = Arc::new(CircuitBreakerManager::new(circuit_config));
    let peer_registry = PeerRegistry::new(cb_mgr);

    let base = EdgeServiceBuilder::new()
        .with_bridge_channel(tx)
        .with_peer_registry(peer_registry)
        .with_preset(sweetmcp::edge::core::builder::BuilderPreset::Development);

    // Use build_multiple with two basic configs
    let cfg1 = Arc::new(sweetmcp::config::Config::default());
    let cfg2 = Arc::new(sweetmcp::config::Config::default());

    let services = EdgeServiceBuilder::build_multiple(base, vec![cfg1, cfg2])?;
    assert_eq!(services.len(), 2);

    Ok(())
}
