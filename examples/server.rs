use std::sync::{Arc, Mutex};
use SH_HTTP_Server::{SmartHouseServer};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let house = Arc::new(Mutex::new(smart_house::SmartHouse::new()));
    SmartHouseServer::new(house).await
}