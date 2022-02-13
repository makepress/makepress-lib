use warp::Rejection;

use crate::{MakepressManager, CreateInfo};


macro_rules! create_handler {
    ($($peices:tt)*) => {__internal_create_handler!{@start $($peices)*}};
}

macro_rules! __internal_create_handler {
    (@start) => {};
    (@start $first:tt $($tail:tt)*) => {__internal_create_handler!{@munch $first $($tail)*}};
    (@munch $name:ident($type1:ty, $type2:ty) => $method:ident, $($tail:tt)*) => {
        pub(crate) async fn $name<T: MakepressManager>(
            param1: $type1,
            param2: $type2,
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method(param1, param2).await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e))
            }
        }
        __internal_create_handler!{@munch $($tail)*}
    };
    (@munch $name:ident($type:ty) => $method:ident, $($tail:tt)*) => {
        pub(crate) async fn $name<T: MakepressManager>(
            param: $type,
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method(param).await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e))
            }
        }
        __internal_create_handler!{@munch $($tail)*}
    };
    (@munch $name:ident => $method:ident, $($tail:tt)*) => {
        pub(crate) async fn $name<T: MakepressManager>(
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method().await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e))
            }
        }
        __internal_create_handler!{@munch $($tail)*}
    };
    (@munch $name:ident($type:ty) => $method:ident) => {
        pub(crate) async fn $name<T: MakepressManager>(
            param: $type,
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method(param).await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e))
            }
        }
    };
    (@munch $name:ident => $method:ident) => {
        pub(crate) async fn $name<T: MakepressManager>(
            param: String,
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method(param).await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e))
            }
        }
    };
    (@munch) => {};
}

create_handler!(
    list_instances => list,
    create_instance(String, CreateInfo) => create,
    get_instance(String) => get,
    start_instance(String) => start,
    stop_instance(String) => stop,
    destroy_instance(String) => destroy,
    create_backup(String) => start_backup,
    check_backup(uuid::Uuid) => check_backup,
    cancel_backup(uuid::Uuid) => cancel_backup
);
