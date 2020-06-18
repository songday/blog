use async_std::task;

use blog::{server, result};

fn main() -> result::Result<()> {
    println!("Web server");
    task::block_on(server::start("127.0.0.1:9270"))
}
