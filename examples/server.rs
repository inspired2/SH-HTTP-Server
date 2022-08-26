use std::sync::{Arc, Mutex};
use sh_http_server::SmartHouseServer;
use smart_house::{PowerSocket, PowerSocketState, Room};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let mut house = smart_house::SmartHouse::new();
    house.try_add_room(Room::with_name("hall")).unwrap();
    house.try_add_device("hall", "socket").unwrap();
    
    
    let mut devices = smart_house::SmartDeviceList::new();
    let socket = smart_house::SmartDevice::Socket(PowerSocket { name: "socket".into(), state: PowerSocketState::NotPowered, description: "".into(), power_consumption: 0 });
    devices.add_device("hall", socket).ok();
    
    let house = Arc::new(Mutex::new(house));
    let provider = Arc::new(devices);
    SmartHouseServer::run(house, provider).await
}
