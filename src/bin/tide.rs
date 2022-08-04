use data::Store;
use tide::{Body,Request,Response};
use tokio::sync::RwLock;
use std::sync::Arc;

type Db = Arc<RwLock<Store>>;

async fn get_all_scans(req: Request<Db>) -> Result<Body, tide::Error> {
    let store = req.state();
    let res = store.read().await.get_all();

    Body::from_json(&res)
}

async fn get_scan(req: Request<Db>) -> Result<Body, tide::Error> {
    let store = req.state();
    let ip: &str = req.param("ip")?;
    let port: i16 = req.param("port")?.parse()?;

    let res = store.read().await.get_record(ip, port);

    Body::from_json(&res)
}

async fn create_scan(mut req: Request<Db>) -> tide::Result<tide::Response> {
    let scan = req.body_json().await?;
    let store = req.state();
    match store.write().await.insert_record(scan) {
        Err(_) => Ok(Response::builder(tide::StatusCode::BadRequest).build()),
        Ok(_) => Ok(Response::builder(tide::StatusCode::Created).build()),
    }
}

async fn update_scan(mut req: Request<Db>) -> tide::Result<tide::Response> {
    let scan = req.body_json().await?;
    let store = req.state();
    match store.write().await.update_record(scan) {
        Err(_) => Ok(Response::builder(tide::StatusCode::BadRequest).build()),
        Ok(_) => Ok(Response::builder(tide::StatusCode::Ok).build()),
    }
}

async fn delete_scan(req: Request<Db>) -> tide::Result<tide::Response> {
    let store = req.state();
    let ip: &str = req.param("ip")?;
    let port: i16 = req.param("port")?.parse()?;

    match store.write().await.delete_record(ip, port) {
        Err(_) => Ok(Response::builder(tide::StatusCode::BadRequest).build()),
        Ok(_) => Ok(Response::builder(tide::StatusCode::Ok).build()),
    }
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let store: Db = Arc::new(RwLock::new(Store::new()));

    let mut app = tide::new();

    app.at("/v1").nest({
        let mut scans = tide::with_state(store);
        scans.at("/scans").get(get_all_scans).post(create_scan).put(update_scan);
        scans.at("/scans/:ip/:port").get(get_scan).delete(delete_scan);
        scans
    });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
