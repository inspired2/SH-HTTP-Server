mod handlers;

use std::{sync::{Arc, Mutex}, marker::PhantomData};
use actix_web::{web::{Data, self}, App, HttpServer};
use smart_house::*;

pub struct SmartHouseServer<T> {
    _inner: PhantomData<T>
}
#[derive(Clone)]
pub struct State<T> {
    house: Arc<Mutex<SmartHouse>>,
    provider: Arc<T>
}

impl<T: DeviceInfoProvider + Send + Sync + 'static> SmartHouseServer<T> {
    pub async fn run(house: Arc<Mutex<SmartHouse>>, provider: Arc<T>) -> Result<(), std::io::Error> {
        let state = Data::new(State {house: house.clone(), provider: provider.clone()});
        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .service(web::resource("/add_room").route(web::post().to(handlers::add_room::<T>)))
                .service(web::resource("/add_device").route(web::post().to(handlers::add_device::<T>)))
                .service(web::resource("/report").route(web::get().to(handlers::report::<T>)))
                .service(web::resource("/rooms").route(web::get().to(handlers::rooms::<T>)))
                .service(web::resource("/devices/{room}").route(web::get().to(handlers::devices::<T>)))
                .service(web::resource("/{room}").route(web::delete().to(handlers::remove_room::<T>)))
                .service(web::resource("/{room}/{device}").route(web::delete().to(handlers::remove_device::<T>)))

        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}
