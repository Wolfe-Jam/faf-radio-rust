use faf_radio_rust::{RadioClient, RadioConfig};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📻 Radio Protocol - Basic Example\n");

    // Create Radio client
    let url = "wss://faf-beacon.wolfejam2020.workers.dev/radio";
    let config = RadioConfig::new(url);
    let mut radio = RadioClient::new(config);

    println!("Connecting to Radio Protocol...");
    radio.connect().await?;

    // Wait for connection to stabilize
    sleep(Duration::from_secs(1)).await;

    println!("\nTuning to 91.0 FM (nelly)...");
    radio.tune(vec!["91.0".to_string()]).await?;

    println!("\nListening for broadcasts (30 seconds)...");
    println!("💡 Trigger a broadcast with:");
    println!("   curl -X POST https://mcpaas.live/beacon/nelly/write \\");
    println!("     -H \"Content-Type: application/json\" \\");
    println!("     -d '{{\"entry\": {{\"message\": \"Hello from Radio!\"}}}}'");
    println!();

    // Listen for 30 seconds
    sleep(Duration::from_secs(30)).await;

    println!("\nDisconnecting...");
    radio.disconnect().await?;

    println!("✅ Example complete!");
    Ok(())
}
