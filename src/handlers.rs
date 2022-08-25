use std::sync::{Arc, Mutex};

use actix_web::{ web::{ self, Data, Json}, HttpResponse};
use crate::SmartHouse as House;
use crate::serde::{ Deserialize, Serialize};
type SmartHouse = Data<Arc<Mutex<House>>>;

pub async fn add_room(house:SmartHouse, room: Json<NewRoom>) -> HttpResponse {
    let room = smart_house::Room::with_name(&room.name.to_owned());
    if let Err(e) = house.lock().unwrap().try_add_room(room) {
        println!("error adding a room. {:?}", e);
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Room successfully added")
}
pub async fn add_device(house: SmartHouse, device: Json<NewDevice>) -> HttpResponse {
    let NewDevice {room, device } = device.into_inner();
    if let Err(e) = house.lock().unwrap().try_add_device(&room, &device) {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Device successfully added")
}

pub async fn remove_device(house: SmartHouse, path: web::Path<(String, String)>) -> HttpResponse {
    let (room, device) = path.into_inner();
    if let Err(e) = house.lock().unwrap().try_remove_device(&room, &device) {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Device successfully removed")
}

pub async fn remove_room(house: SmartHouse, path: web::Path<String>) -> HttpResponse {
    let room = path.into_inner();
    if let Err(e) = house.lock().unwrap().try_remove_room(&room) {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Room successfully deleted")
}

pub async fn state(house: SmartHouse) -> HttpResponse {
    let rooms_mtx = house.lock().unwrap();
    let rooms: Vec<String> = rooms_mtx.get_rooms().into_iter().map(|s| s.to_owned()).collect();
    drop(rooms_mtx);
    let house = house.lock().unwrap();
    let mut devices: Vec<&str> = Vec::new();
    for room in &rooms {
        let mut dvcs = house.get_devices(room);
        devices.append(&mut dvcs)
    }
    println!("sending state: {:?}", devices);
    HttpResponse::Ok().json(devices)
}
#[derive(Serialize, Deserialize)]
pub struct NewRoom {
    name: String
}

#[derive(Serialize, Deserialize)]
pub struct NewDevice {
    device: String,
    room: String
}