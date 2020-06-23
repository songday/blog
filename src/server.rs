use std::convert::Infallible;
use std::net::SocketAddr;

use warp::{self, Filter, Reply, reject, Rejection};

use crate::{controller, db::{self, DataSource}, result::{self, Error}, service};
use crate::model::User;

fn with_db(datasource: DataSource) -> impl Filter<Extract = (DataSource,), Error = Infallible> + Clone {
    warp::any().map(move || datasource.clone())
}

fn check_auth() -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    warp::header::<String>("x-auth").and_then(|token: String| async move {
        service::check_auth(&token).map_err(|e| reject::custom(e))
    })
}

pub async fn start(address:&str) -> result::Result<()> {
    let datasource = db::get_datasource().await?;

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::path::end())
        .and_then(controller::index);
    let about = warp::get()
        .and(warp::path("about"))
        .and(warp::path::end())
        .and_then(controller::about);
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

    let routes = index.or(about).or(blog_list).or(blog_save)
            .with(warp::cors().allow_any_origin())
            .recover(controller::handle_rejection)
        ;
    let addr = address.parse::<SocketAddr>()?;
    warp::serve(routes).run(addr).await;
    Ok(())
}