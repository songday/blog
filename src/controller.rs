use std::convert::Infallible;
use std::result::Result;

use warp::{
    http::{Response, StatusCode},
    reject, Rejection, Reply,
};

use crate::db::DataSource;
use crate::model::{Blog, User};
use crate::result::{Error, ErrorResponse};
use crate::service;

// lazy_static_include_str!(INDEX_PAGE_BYTES, "./src/asset/index.html");

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else if let Some(e) = err.find::<crate::result::Error>() {
        match e {
            Error::NotAuthed => {
                code = StatusCode::BAD_REQUEST;
                message = "需要登录";
            }
            Error::LoginFailed => {
                code = StatusCode::BAD_REQUEST;
                message = "登录失败";
            }
            Error::SaveBlogFailed => {
                code = StatusCode::BAD_REQUEST;
                message = "保存博客失败";
            }
            Error::CannotFoundBlog => {
                code = StatusCode::BAD_REQUEST;
                message = "未找到博客";
            }
            // Error::DBQueryError(_) => {
            //     code = StatusCode::BAD_REQUEST;
            //     message = "Could not Execute request";
            // }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

pub async fn index() -> Result<impl Reply, Rejection> {
    let s = include_str!("asset/page/index.html");
    Ok(warp::reply::html(s))
}

pub async fn about() -> Result<impl Reply, Rejection> {
    let s = include_str!("asset/page/about.html");
    Ok(warp::reply::html(s))
}

pub async fn verify_image() -> Result<Response<Vec<u8>>, Rejection> {
    let numbers = crate::util::num::rand_numbers(4);
    let b = crate::image::image::gen_verify_image(numbers.as_slice());
    warp::http::Response::builder()
        .header("content-type", "image/png")
        .body(b)
        .map_err(|e| {
            eprintln!("{}", e);
            reject::custom(Error::SerdeError)
        })
}

pub async fn user_login(datasource: DataSource, user: User) -> Result<impl Reply, Rejection> {
    let u = service::user_login(&datasource, &user.username, &user.password)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(warp::reply::json(&u))
}

pub async fn blog_list(datasource: DataSource, page_num: i32) -> Result<impl Reply, Rejection> {
    let result = service::blog_list(&datasource, page_num).await;
    match result {
        Ok(list) => Ok(warp::reply::json(&list)),
        Err(e) => Err(reject::custom(e)),
    }
}

pub async fn blog_save(
    _user: User,
    datasource: DataSource,
    blog: Blog,
) -> Result<impl Reply, Rejection> {
    let blog = service::blog_save(&datasource, blog)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(warp::reply::json(&blog))
}

pub async fn blog_show(datasource: DataSource, id: u64) -> Result<impl Reply, Rejection> {
    let blog = datasource
        .blog_show(id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(warp::reply::json(&blog))
}
