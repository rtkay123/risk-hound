use anyhow::Result;
use hound_services::tracing::Telemetry;

#[tokio::main]
async fn main() -> Result<()> {
    let _tracing = Telemetry::builder().build();

    risk_hound::serve().await
}
