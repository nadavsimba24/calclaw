// 🚀 Calclaw Orchestration System - Main Entry Point

use calclaw::orchestration_api;

#[tokio::main]
async fn main() {
    println!("🚀 Calclaw Orchestration System");
    println!("================================");
    println!("");
    
    // הפעל את שרת האורקסטרציה
    orchestration_api::run_orchestration_server(3001).await;
}
