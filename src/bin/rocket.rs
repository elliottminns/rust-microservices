#[macro_use] extern crate rocket;

use data::{Scan,Store};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::sync::Arc;
use rocket::tokio::sync::RwLock;

type Db = Arc<RwLock<Store>>;

#[get("/")]
async fn get_all_scans(store: &State<Db>) -> Json<Vec<Scan>> {
    Json(store.read().await.get_all())
}

#[get("/<ip>/<port>")]
async fn get_scan(store: &State<Db>, ip: &str, port: i16) -> Json<Option<Scan>> {
    Json(store.read().await.get_record(ip, port))
}

#[post("/", data="<scan>")]
async fn create_scan(store: &State<Db>, scan: Json<Scan>) -> Status {
    let s = scan.into_inner().clone();
    match store.write().await.insert_record(s) {
        Err(_) => Status::BadRequest,
        Ok(_) => Status::Created,
    }
}

#[put("/", data="<scan>")]
async fn update_scan(store: &State<Db>, scan: Json<Scan>) -> Status {
    let s = scan.into_inner().clone();
    match store.write().await.update_record(s) {
        Err(_) => Status::BadRequest,
        Ok(_) => Status::Ok,
    }
}

#[delete("/<ip>/<port>")]
async fn delete_scan(store: &State<Db>, ip: &str, port: i16) -> Status {
    match store.write().await.delete_record(ip, port) {
        Err(_) => Status::BadRequest,
        Ok(_) => Status::Ok,
    }
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("port", 8080));

    rocket::custom(figment)
        .manage(Arc::new(RwLock::new(Store::new())))
        .mount("/v1/scans", routes![
               get_all_scans, get_scan, create_scan, update_scan, delete_scan,
        ])
}
