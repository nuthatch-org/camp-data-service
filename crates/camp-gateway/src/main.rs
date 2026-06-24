//! camp-gateway — Horizon TAP v2 (GraphTally) payment layer in front of the camp REST API.
//!
//! All payment machinery (receipt validation, RAV aggregation, on-chain collection,
//! persistence, the TAP-gated reverse proxy) lives in `horizon-core`. This binary loads
//! config and hands off: a consumer sends a signed `TAP-Receipt` header, the gateway
//! verifies + meters it and proxies the request to the upstream camp instance.
//!
//! NOTE: per-endpoint compute-unit pricing (the former `pricing.rs`) was never enforced
//! gateway-side — consumers set receipt values per the published schedule. horizon-core
//! does not yet express per-path pricing; see the porting notes in lodestone.
//!
//! DISCLAIMER: experimental community project. Not affiliated with or endorsed by
//! The Graph Foundation or Edge & Node.

use horizon_core::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "camp_gateway=info,horizon_core=info".into()),
        )
        .init();

    let config = Config::load()?;
    tracing::info!(
        upstream = %config.backend.upstream_url,
        data_service = %config.tap.data_service_address,
        "camp-gateway starting — Camp data on Horizon"
    );

    horizon_core::run(config).await
}
