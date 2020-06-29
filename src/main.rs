use blog::{result, server};

#[tokio::main]
async fn main() {
    println!("Web server");
    server::start("127.0.0.1:9270").await;
}
