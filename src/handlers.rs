use std::sync::{Arc, Mutex};

use actix_web::{web::{self,Data, Json}, HttpResponse};
use crate::SmartHouse;
use crate::serde::{ Deserialize, Serialize};

pub async fn add_room(house:Data<Arc<Mutex<SmartHouse>>>, room: Json<NewRoom>) -> HttpResponse {
    let room = smart_house::Room::with_name(&room.name.to_owned());
    if let Err(e) = house.lock().unwrap().try_add_room(room) {
        println!("error adding a room. {:?}", e);
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Room successfully added")
}

#[derive(Serialize, Deserialize)]
pub struct NewRoom {
    name: String
}