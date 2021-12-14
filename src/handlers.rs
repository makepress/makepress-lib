use warp::Rejection;

use crate::MakepressManager;

macro_rules! create_handler {
    ($($name:ident => $method:ident),* ,) => {
        create_handler!($($name => $method),*);
    };
    ($($name:ident => $method:ident),*) => {
        $(create_handler!($name => $method);)*
    };
    ($name:ident => $method:ident) => {
        pub(crate) async fn $name<T: MakepressManager>(
            name: String,
            manager: T
        ) -> Result<impl warp::Reply, Rejection> {
            match manager.$method(name).await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e))
            }
        }
    };
}

create_handler!(
    create_container => create,
    get_container => get,
);