use warp::Rejection;

use crate::MakepressManager;

pub(crate) async fn get_container<T: MakepressManager>(
    name: String,
    manager: T,
) -> Result<impl warp::Reply, Rejection> {
    match manager.get(name).await {
        Ok(info) => Ok(warp::reply::json(&info)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub(crate) async fn create_container<T: MakepressManager>(
    name: String,
    manager: T,
) -> Result<impl warp::Reply, Rejection> {
    match manager.create(name).await {
        Ok(info) => Ok(warp::reply::json(&info)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
