use warp::Rejection;

use crate::MakepressManager;

macro_rules! create_handler {
    ($($peices:tt)*) => {__internal_create_handler!{@start $($peices)*}};
}

macro_rules! __internal_create_handler {
    (@start) => {};
    (@start $first:tt $($tail:tt)*) => {__internal_create_handler!{@munch $first $($tail)*}};
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
            param: String,
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method(param).await {
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
    create_instance => create,
    get_instance => get,
    start_instance => start,
    stop_instance => stop,
    destroy_instance => destroy,
    create_backup => start_backup,
    check_backup(uuid::Uuid) => check_backup,
    cancel_backup(uuid::Uuid) => cancel_backup
);
