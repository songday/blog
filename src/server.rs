use tide::{self, Server};

use crate::{controller, db::{self, DataSource}, result};

pub async fn start(address:&str) -> result::Result<()> {
    let mut app = tide::with_state(db::get_datasource().await?);
    setup_route(&mut app);
    app.listen(address).await?;
    Ok(())
}

fn setup_route(app: &mut Server<DataSource>) {
    app.at("/").get(controller::index);
    app.at("/index").get(controller::index);
    app.at("/about").get(controller::about);
    app.at("/blog/save").get(controller::blog_save);
    // app.at("/items").get(handler::get_items);
    // app.at("/s").get(handler::scrape);
    // app.at("/hello").get(|_| async move { Ok("Hello, world!") });
}
