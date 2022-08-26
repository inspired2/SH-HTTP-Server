mod handlers;

use std::sync::{Arc, Mutex};
use actix_web::{web::Data, App, HttpServer};
use smart_house::*;

pub struct SmartHouseServer;

impl SmartHouseServer {
    pub async fn run(house: Arc<Mutex<SmartHouse>>) -> Result<(), std::io::Error> {
        HttpServer::new(move || {
            let house = house.clone();
            App::new()
                .app_data(Data::new(house))
                .service(handlers::add_room)
                .service(handlers::add_device)
                .service(handlers::remove_room)
                .service(handlers::remove_device)
                .service(handlers::state)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}
