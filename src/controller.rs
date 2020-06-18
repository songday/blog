use tide::{Request, Response, StatusCode, Result};
use tide::http::Body;

use crate::db::DataSource;
use crate::model::Blog;

lazy_static_include_bytes!(INDEX_PAGE_BYTES, "./src/asset/index.html");

pub async fn index(_req: Request<DataSource>) -> Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(*INDEX_PAGE_BYTES);
    Ok(res)
}

pub async fn about(_req: Request<DataSource>) -> Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    let b = include_bytes!("asset/about.html");
    res.set_body(Body::from_bytes(b.to_vec()));
    Ok(res)
}

pub async fn blog_list(req: Request<DataSource>) -> Result<Response> {
    // 下面两行
    // let p : CatelogRequest = req.body_json().await.unwrap();
    let p = match req.query::<Blog>() {
        Ok(r) => r,
        Err(e) => {
            println!("{:?}", e);
            return Result::Err(e)
        }
    };
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(format!("{:?}", p));
    Ok(res)
}

pub async fn blog_save(mut req: Request<DataSource>) -> Result<Response> {
    let mut blog: Blog = req.body_json().await?;
    blog.save(req.state()).await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(format!("{:?}", blog));
    Ok(res)
}
