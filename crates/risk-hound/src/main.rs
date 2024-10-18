use hound_services::tracing::Telemetry;
use tracing::info;

fn main() {
    let tracing = Telemetry::builder().build();
    info!("Hello, world!");
}
