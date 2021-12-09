use warp::Filter;

use crate::MakepressManager;

pub fn all<T: MakepressManager + Send + Sync>(
    manager: T,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_container(manager.clone()).or(create_container(manager))
}

fn with_manager<T: MakepressManager + Send>(
    manager: T,
) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

fn get_container<T: MakepressManager + Send + Sync>(
    manager: T,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get" / String)
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(crate::handlers::get_container)
}

fn create_container<T: MakepressManager + Send + Sync>(
    manager: T,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create" / String)
        .and(warp::post())
        .and(with_manager(manager))
        .and_then(crate::handlers::create_container)
}
