use faf_radio_rust::{RadioClient, RadioConfig};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📻 Radio Protocol - Multi-AI Example\n");
    println!("Demonstrating 3 AIs tuned to the same frequency\n");

    let url = "wss://faf-beacon.wolfejam2020.workers.dev/radio";

    // Create 3 AI clients
    let mut claude = RadioClient::new(RadioConfig::new(url));
    let mut grok = RadioClient::new(RadioConfig::new(url));
    let mut gemini = RadioClient::new(RadioConfig::new(url));

    println!("🤖 Connecting Claude...");
    claude.connect().await?;
    sleep(Duration::from_millis(500)).await;

    println!("🤖 Connecting Grok...");
    grok.connect().await?;
    sleep(Duration::from_millis(500)).await;

    println!("🤖 Connecting Gemini...");
    gemini.connect().await?;
    sleep(Duration::from_secs(1)).await;

    println!("\n📡 Tuning all AIs to 91.0 FM (nelly)...");
    claude.tune(vec!["91.0".to_string()]).await?;
    grok.tune(vec!["91.0".to_string()]).await?;
    gemini.tune(vec!["91.0".to_string()]).await?;

    println!("\n✅ All AIs tuned to 91.0 FM!");
    println!("\n💡 When someone writes to nelly:");
    println!("   curl -X POST https://mcpaas.live/beacon/nelly/write \\");
    println!("     -H \"Content-Type: application/json\" \\");
    println!("     -d '{{\"entry\": {{\"fact\": \"Elephants never forget!\"}}}}'");
    println!("\n📻 ALL 3 AIs receive the broadcast simultaneously!");
    println!("   - 99% cost reduction vs 3 separate API calls");
    println!("   - Zero context drift");
    println!("   - One write, infinite reads\n");

    println!("Listening for 60 seconds...\n");
    sleep(Duration::from_secs(60)).await;

    println!("\nDisconnecting all AIs...");
    claude.disconnect().await?;
    grok.disconnect().await?;
    gemini.disconnect().await?;

    println!("✅ Multi-AI example complete!");
    Ok(())
}
