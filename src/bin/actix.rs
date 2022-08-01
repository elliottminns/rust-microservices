use data::{Store,Scan};
use actix_web::{get,post,put,delete,App,HttpServer,HttpResponse,web};
use actix_web::web::Data;
use tokio::sync::Mutex;

#[get("")]
async fn get_all_scans(store: Data<Mutex<Store>>) -> HttpResponse {
    HttpResponse::Ok().json(store.lock().await.get_all())
}

#[get("/{ip}/{port}")]
async fn get_scan(store: Data<Mutex<Store>>, path_param: web::Path<(String, i16)>) -> HttpResponse {
    let params = path_param.into_inner();
    let ip = params.0;
    let port = params.1;

    HttpResponse::Ok().json(store.lock().await.get_record(&ip, port))
}

#[post("")]
async fn create_scan(store: Data<Mutex<Store>>, item: web::Json<Scan>) -> HttpResponse {
    match store.lock().await.insert_record(item.0) {
        Err(x) => HttpResponse::BadRequest().body(x),
        Ok(_) => HttpResponse::Created().finish()
    }
}

#[put("")]
async fn update_scan(store: Data<Mutex<Store>>, item: web::Json<Scan>) -> HttpResponse {
    match store.lock().await.update_record(item.0) {
        Err(x) => HttpResponse::BadRequest().body(x),
        Ok(_) => HttpResponse::Ok().finish()
    }
}

#[delete("/{ip}/{port}")]
async fn delete_scan(store: Data<Mutex<Store>>, path_param: web::Path<(String, i16)>) -> HttpResponse {
    let params = path_param.into_inner();
    let ip = params.0;
    let port = params.1;

    match store.lock().await.delete_record(&ip, port) {
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
            .service(
                web::scope("/v1").service(
                    web::scope("/scans")
                    .service(get_all_scans)
                    .service(get_scan)
                    .service(create_scan)
                    .service(update_scan)
                    .service(delete_scan)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
