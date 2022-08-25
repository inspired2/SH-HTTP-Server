mod handlers;

use std::sync::{Arc, Mutex};

use actix_web::{ web::{self, Data}, App, HttpServer };
use smart_house::*;
use serde;

pub struct SmartHouseServer;

impl SmartHouseServer {
    pub async fn new(house: Arc<Mutex<SmartHouse>>) -> Result<(), std::io::Error> {
        HttpServer::new( move || {
            let house = house.clone();
            App::new()
                .app_data(Data::new(house))
                .service(web::resource("/add_room").route((web::post()).to(handlers::add_room)))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }

}