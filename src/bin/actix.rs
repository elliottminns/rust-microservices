use data::{Store,Scan};
use actix_web::{get,post,put,delete,App,HttpServer,HttpResponse,web};
use actix_web::web::Data;
use std::sync::Mutex;

#[get("/v1/scans")]
async fn get_all_scans(store: Data<Mutex<Store>>) -> HttpResponse {
    HttpResponse::Ok().json(store.lock().unwrap().get_all())
}

#[get("/v1/scans/{ip}/{port}")]
async fn get_scan(store: Data<Mutex<Store>>, path_param: web::Path<(String, i16)>) -> HttpResponse {
    let params = path_param.into_inner();
    let ip = params.0;
    let port = params.1;

    HttpResponse::Ok().json(store.lock().unwrap().get_record(&ip, port))
}

#[post("/v1/scans")]
async fn create_scan(store: Data<Mutex<Store>>, item: web::Json<Scan>) -> HttpResponse {
    match store.lock().unwrap().insert_record(item.0) {
        Err(x) => HttpResponse::BadRequest().body(x),
        Ok(_) => HttpResponse::Created().finish()
    }
}

#[put("/v1/scans")]
async fn update_scan(store: Data<Mutex<Store>>, item: web::Json<Scan>) -> HttpResponse {
    match store.lock().unwrap().update_record(item.0) {
        Err(x) => HttpResponse::BadRequest().body(x),
        Ok(_) => HttpResponse::Ok().finish()
    }
}

#[delete("/v1/scans/{ip}/{port}")]
async fn delete_scan(store: Data<Mutex<Store>>, path_param: web::Path<(String, i16)>) -> HttpResponse {
    let params = path_param.into_inner();
    let ip = params.0;
    let port = params.1;

    match store.lock().unwrap().delete_record(&ip, port) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(x) => HttpResponse::BadRequest().body(x)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let store = Data::new(Mutex::new(Store::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .app_data(web::JsonConfig::default())
            .service(get_all_scans)
            .service(get_scan)
            .service(create_scan)
            .service(update_scan)
            .service(delete_scan)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
