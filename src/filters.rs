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

macro_rules! create_filter {
    ($($name:ident => {$path:expr, $method:ident, $handler:ident}),*,) => {
        create_filter!($($name => {$path, $method, $handler}),*);
    };
    ($($name:ident => {$path:expr, $method:ident, $handler:ident}),*) => {
        $(create_filter!($name => {$path, $method, $handler}));*
    };
    ($name:indent => {$path:expr, $method:ident, $handler:ident}) => {
        fn $name<T: MakepressManager + Send + Sync>(
            manager: T,
        ) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
            warp::path!($path)
                .and(warp::$method())
                .and(with_manager(manager))
                .and_then(crate::handlers::$handler)
        }
    };
}

create_filter!(
    get_container => {"get" / String, get, get_container},
    create_container => {"create" / String, post, create_container},
);