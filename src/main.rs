use stable_eyre::eyre::Result;
use std::env;

mod aws;
mod prometheus;

#[tokio::main]
async fn main() -> Result<()> {
    stable_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("TMD AWS Exporter starting");

    // Get station ID from environment variable
    let station_id: u32 = env::var("AWS_STATION_ID")
        .unwrap_or_else(|_| "104".to_string())
        .parse()
        .expect("AWS_STATION_ID must be a valid number");

    tracing::info!("Monitoring station ID: {}", station_id);

    // Create AWS client
    let aws_client = aws::AwsClient::new(station_id);

    // Create Prometheus exporter
    let addr = "0.0.0.0:9100"
        .parse()
        .expect("Failed to parse bind address");
    let exporter = prometheus::PrometheusExporter::new(addr)?;

    tracing::info!("Exporter listening on {}", addr);

    loop {
        // Block until Prometheus scrapes us, then hold the guard while we update.
        // block_in_place lets us call this blocking API from inside an async context.
        let _guard = tokio::task::block_in_place(|| exporter.wait_request());

        tracing::info!("Received scrape request, fetching AWS data");

        match aws_client.get_observation_now().await {
            Ok(Some(obs)) => {
                tracing::debug!("Fetched observation for station {} ({})", obs.id, obs.sname);
                exporter.update_from_observation(&obs);
            }
            Ok(None) => {
                tracing::warn!("No observation data available for station {}", station_id);
            }
            Err(e) => {
                tracing::error!("Error fetching AWS data: {:#}", e);
            }
        }

        drop(_guard);
    }
}
