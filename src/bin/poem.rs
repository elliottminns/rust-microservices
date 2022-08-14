use poem::{get,post,put,delete,handler,Route,EndpointExt,Server,Response};
use poem::web::{Path,Data,Json};
use poem::http::StatusCode;
use poem::listener::TcpListener;
use data::Store;
use tokio::sync::RwLock;
use std::sync::Arc;

pub type Db = Arc<RwLock<Store>>;

#[handler]
async fn get_all_scans(store: Data<&Db>) -> Json<Vec<data::Scan>> {
    Json(store.read().await.get_all())
}

#[handler]
async fn get_scan(store: Data<&Db>, path: Path<(String, i16)>) -> Json<Option<data::Scan>> {
    Json(store.read().await.get_record(&path.0.0, path.0.1))
}

#[handler]
async fn create_scan(store: Data<&Db>, scan: Json<data::Scan>) -> Response {
    let status = match store.write().await.insert_record(scan.0.clone()) {
        Err(_) => StatusCode::BAD_REQUEST,
        Ok(_) => StatusCode::CREATED,
    };

    Response::builder().status(status).finish()
}

#[handler]
async fn update_scan(store: Data<&Db>, scan: Json<data::Scan>) -> Response {
    let status = match store.write().await.update_record(scan.0.clone()) {
        Err(_) => StatusCode::BAD_REQUEST,
        Ok(_) => StatusCode::OK,
    };

    Response::builder().status(status).finish()
}

#[handler]
async fn delete_scan(store: Data<&Db>, path: Path<(String, i16)>) -> Response {
    let status = match store.write().await.delete_record(&path.0.0, path.0.1) {
        Err(_) => StatusCode::BAD_REQUEST,
        Ok(_) => StatusCode::OK,
    };

    Response::builder().status(status).finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let store = Arc::new(RwLock::new(Store::new()));

    let scans = Route::new()
        .at("/scans", get(get_all_scans).post(create_scan).put(update_scan))
        .at("/scans/:ip/:port", get(get_scan).delete(delete_scan))
        .data(store);

    let app = Route::new().nest("/v1", scans);
                                
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await
}
