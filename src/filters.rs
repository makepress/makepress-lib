use warp::Filter;

use crate::MakepressManager;
use uuid::Uuid;

macro_rules! all_body {
    ($($pieces:tt)*) => {__internal_all_body!{@start $($pieces)*}};
}

macro_rules! __internal_all_body {
    (@start) => {};
    (@start $first:tt $(| $tail:tt)*) => {
        pub fn all<T: MakepressManager + Send + Sync>(
            manager: T,
        ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            __internal_all_body!{@m manager [$first] [$(| $tail)*]}
        }
    };
    (@m $manager:ident [$cur:tt] []) => {
            $cur($manager)
    };
    (@m $manager:ident [$cur:tt] [| $next:tt $(| $tail:tt)*]) => {
            __internal_all_body!{@munch $manager $cur($manager.clone()); [$next] [$(| $tail)*]}
    };
    (@munch $manager:ident $sum:expr; [$cur:tt] [| $next:tt $(| $tail:tt)*]) => {
        __internal_all_body!{@munch $manager $sum.or($cur($manager.clone())); [$next] [$(| $tail)*]}
    };
    (@munch $manager:ident $sum:expr; [$cur:tt] []) => {
        __internal_all_body!{@last $manager $sum; $cur}
    };
    (@last $manager:ident $sum:expr; $end:tt) => {
        $sum.or($end($manager))
    };
}

all_body!(
    create_instance
        | get_instance
        | start_instance
        | stop_instance
        | destroy_instance
        | start_backup
        | check_backup
        | cancel_backup
        | list_instances
);

fn with_manager<T: MakepressManager + Send>(
    manager: T,
) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

macro_rules! filter {
    ($($pieces:tt)*) => (__internal_filter!(@start $($pieces)*););
}

macro_rules! __internal_filter {
    (@start) => ();
    (@start $first:tt $($tail:tt)*) => (
        __internal_filter!(@method warp::any(); [$first] [$($tail)*]);
    );
    (@method $sum:expr; [POST] [$next:tt $($tail:tt)*]) => (
        __internal_filter!(@path $sum.and(warp::post()); [$next] [$($tail)*]);
    );
    (@method $sum:expr; [GET] [$next:tt $($tail:tt)*]) => (
        __internal_filter!(@path $sum.and(warp::get()); [$next] [$($tail)*]);
    );
    (@method $sum:expr; [PUT] [$next:tt $($tail:tt)*]) => (
        __internal_filter!(@path $sum.and(warp::put()); [$next] [$($tail)*]);
    );
    (@method $sum:expr; [DELETE] [$next:tt $($tail:tt)*]) => (
        __internal_filter!(@path $sum.and(warp::delete()); [$next] [$($tail)*]);
    );
    (@path $sum:expr;  [$cur:tt] [/ $next:tt $($tail:tt)*]) => (
        __internal_filter!(@path $sum.and(__internal_filter!(@segment $cur)); [$next] [$($tail)*]);
    );
    (@path $sum:expr; [$cur:tt] [= $next:tt $($tail:tt)*]) => (
        __internal_filter!(@name $sum.and(__internal_filter!(@segment $cur)); [$next] [$($tail)*]);
    );
    (@name $sum:expr; [$name:ident] [=> $handler:ident]) => (
        fn $name<T: MakepressManager + Send + Sync>(
            manager: T
        ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            $sum.and(with_manager(manager)).and_then(crate::handlers::$handler)
        }
    );
    (@name $sum:expr; [$name:ident] [=> $handler:ident ;]) => (
        fn $name<T: MakepressManager + Send + Sync>(
            manager: T
        ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            $sum.and(with_manager(manager)).and_then(crate::handlers::$handler)
        }
    );
    (@name $sum:expr; [$name:ident] [=> $handler:ident; $($tail:tt)*]) => (
        fn $name<T: MakepressManager + Send + Sync>(
            manager: T
        ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            $sum.and(with_manager(manager)).and_then(crate::handlers::$handler)
        }
        __internal_filter!(@start $($tail)*);
    );
    (@segment $param:ty) => (
        warp::path::param::<$param>()
    );
    (@segment $s:literal) => ({
        #[derive(Clone, Copy)]
        struct __StaticPath;
        impl ::std::convert::AsRef<str> for __StaticPath {
            fn as_ref(&self) -> &str {
                static S: &str = $s;
                S
            }
        }
        warp::path(__StaticPath)
    });
}

filter!(
    GET "instance" = list_instances => list_instances;
    POST "instance" / String = create_instance => create_instance;
    GET "instance" / String = get_instance => get_instance;
    PUT "instance" / String / "start" = start_instance => start_instance;
    PUT "instance" / String / "stop" = stop_instance => stop_instance;
    DELETE "instance" / String = destroy_instance => destroy_instance;
    POST "instance" / String / "backup" = start_backup => create_backup;
    GET "backup" / Uuid = check_backup => check_backup;
    DELETE "backup" / Uuid = cancel_backup => cancel_backup;
);
