use std::{convert::Infallible, net::SocketAddr};

use futures::future::Future;
use tokio::sync::oneshot::Receiver;

use warp::{self, reject, Filter, Rejection};

use crate::{
    controller,
    db::{self, DataSource},
    model::User,
    result::{self},
    service,
};

fn with_db(datasource: DataSource) -> impl Filter<Extract = (DataSource,), Error = Infallible> + Clone {
    warp::any().map(move || datasource.clone())
}

fn check_auth() -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    warp::header::<String>("x-auth")
        .and_then(|token: String| async move { service::check_auth(&token).map_err(|e| reject::custom(e)) })
}

pub async fn create_server(
    address: &str,
    receiver: Receiver<()>,
) -> result::Result<(impl Future<Output = ()> + 'static)> {
    let datasource = crate::db::get_datasource().await?;
    let (_addr, server) = create_warp_server(address, datasource, receiver)?;
    Ok(server)
}

fn create_warp_server(
    address: &str,
    datasource: DataSource,
    receiver: Receiver<()>,
) -> result::Result<(SocketAddr, impl Future<Output = ()> + 'static)> {
    let index = warp::get().and(warp::path::end()).and_then(controller::index);
    let about = warp::get()
        .and(warp::path("about"))
        .and(warp::path::end())
        .and_then(controller::about);
    let user_login = warp::post()
        .and(warp::path("user"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(with_db(datasource.clone()))
        .and(warp::body::json())
        .and_then(controller::user_login);
    let verify_image = warp::get()
        .and(warp::path("tool"))
        .and(warp::path("verify-image"))
        .and(warp::path::end())
        .and(warp::query::<String>())
        .and_then(controller::verify_image);
    let blog_list = warp::get()
        .and(warp::path("blog"))
        .and(warp::path("list"))
        .and(warp::path::end())
        .and(with_db(datasource.clone()))
        .and(warp::query::<i32>())
        .and_then(controller::blog_list);
    let blog_save = warp::post()
        .and(warp::path("blog"))
        .and(warp::path("save"))
        .and(warp::path::end())
        .and(check_auth())
        .and(with_db(datasource.clone()))
        .and(warp::body::json())
        .and_then(controller::blog_save);
    let blog_show = warp::post()
        .and(warp::path("blog"))
        .and(warp::path("show"))
        .and(warp::path::end())
        .and(with_db(datasource.clone()))
        .and(warp::query::<u64>())
        .and_then(controller::blog_show);

    let routes = index
        .or(about)
        .or(user_login)
        .or(verify_image)
        .or(blog_list)
        .or(blog_save)
        .or(blog_show)
        .with(warp::cors().allow_any_origin())
        .recover(controller::handle_rejection);
    let addr = address.parse::<SocketAddr>()?;
    Ok(warp::serve(routes).bind_with_graceful_shutdown(addr, async {
        receiver.await.ok();
    }))
}
