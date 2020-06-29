use blog::{result, server};

#[tokio::main]
async fn main() -> result::Result<()> {
    println!("Web server");
    server::start("127.0.0.1:9270").await
}
