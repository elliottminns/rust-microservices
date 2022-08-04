use data::Store;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<Store>>;

mod filters {
    use super::{handlers,Db};
    use data::Scan;
    use warp::{Filter,Reply,Rejection};

    pub fn scans(
        store: Db
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        scans_list(store.clone())
            .or(scan_read(store.clone()))
            .or(scan_create(store.clone()))
            .or(scan_update(store.clone()))
            .or(scan_delete(store.clone()))
    }

    pub fn scans_list(
        store: Db
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("v1" / "scans")
            .and(warp::get())
            .and(with_store(store))
            .and_then(handlers::get_all_scans)
    }

    pub fn scan_read(
        store: Db,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("v1" / "scans" / String / i16)
            .and(warp::get())
            .and(with_store(store))
            .and_then(handlers::get_scan)
    }

    pub fn scan_create(
        store: Db,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("v1" / "scans")
            .and(warp::post())
            .and(json_body())
            .and(with_store(store))
            .and_then(handlers::create_scan)
    }

    pub fn scan_update(
        store: Db,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("v1" / "scans")
            .and(warp::put())
            .and(json_body())
            .and(with_store(store))
            .and_then(handlers::update_scan)
    }

    pub fn scan_delete(
        store: Db,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("v1" / "scans" / String / i16)
            .and(warp::delete())
            .and(with_store(store))
            .and_then(handlers::delete_scan)
    }

    fn with_store(
        store: Db
    ) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || store.clone())
    }

    fn json_body() -> impl Filter<Extract = (Scan,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::Db;
    use data::Scan;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_all_scans(store: Db) -> Result<impl warp::Reply, Infallible> {
        let res = store.read().await.get_all();
        Ok(warp::reply::json(&res))
    }

    pub async fn get_scan(
        ip: String, port: i16, store: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        let res = store.read().await.get_record(&ip, port);
        Ok(warp::reply::json(&res))
    }

    pub async fn create_scan(
        scan: Scan, store: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        match store.write().await.insert_record(scan) {
            Err(_) => Ok(StatusCode::BAD_REQUEST),
            Ok(_) => Ok(StatusCode::CREATED),
        }
    }

    pub async fn update_scan(
        scan: Scan, store: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        match store.write().await.update_record(scan) {
            Err(_) => Ok(StatusCode::BAD_REQUEST),
            Ok(_) => Ok(StatusCode::OK),
        }
    }

    pub async fn delete_scan(
        ip: String, port: i16, store: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        match store.write().await.delete_record(&ip, port) {
            Err(_) => Ok(StatusCode::BAD_REQUEST),
            Ok(_) => Ok(StatusCode::OK),
        }
    }
}


#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(Store::new()));
    let api = filters::scans(store);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
