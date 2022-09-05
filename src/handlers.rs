use crate::State;
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use smart_house::DeviceInfoProvider;

type SmartHouse<T> = Data<State<T>>;

pub async fn add_room<T>(state: SmartHouse<T>, room: Json<NewRoom>) -> HttpResponse {
    let room = smart_house::Room::with_name(&room.name.to_owned());
    if let Err(e) = state.house.lock().unwrap().try_add_room(room) {
        println!("error adding a room. {:?}", e);
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Room successfully added")
}

pub async fn add_device<T>(state: SmartHouse<T>, device: Json<NewDevice>) -> HttpResponse {
    let NewDevice { room, device } = device.into_inner();
    if let Err(e) = state.house.lock().unwrap().try_add_device(&room, &device) {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Device successfully added")
}

pub async fn remove_device<T>(
    state: SmartHouse<T>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (room, device) = path.into_inner();
    if let Err(e) = state
        .house
        .lock()
        .unwrap()
        .try_remove_device(&room, &device)
    {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Device successfully removed")
}

pub async fn remove_room<T>(state: SmartHouse<T>, path: web::Path<String>) -> HttpResponse {
    let room = path.into_inner();
    if let Err(e) = state.house.lock().unwrap().try_remove_room(&room) {
        return HttpResponse::InternalServerError().json(e.to_string());
    }
    HttpResponse::Ok().json("Room successfully deleted")
}

pub async fn rooms<T>(state: SmartHouse<T>) -> HttpResponse {
    let house = state.house.lock().unwrap();
    let rooms: Vec<String> = house
        .get_rooms()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    drop(house);
    HttpResponse::Ok().json(rooms)
}

pub async fn devices<T>(state: SmartHouse<T>, path: web::Path<String>) -> HttpResponse {
    let house = state.house.lock().unwrap();
    match house.get_devices(path.as_str()) {
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        Ok(vec) => {
            let devices: Vec<String> = vec.into_iter().map(|s| s.to_owned()).collect();
            HttpResponse::Ok().json(devices)
        }
    }
}

pub async fn report<T: DeviceInfoProvider>(state: SmartHouse<T>) -> HttpResponse {
    let house = state.house.lock().unwrap();
    let provider = state.provider.as_ref();
    let report = house.get_report(provider);
    println!("generated report: {:?}", &report);
    HttpResponse::Ok().json(report)
}

#[derive(Serialize, Deserialize)]
pub struct NewRoom {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewDevice {
    device: String,
    room: String,
}
