use std::time::Duration;

use tokio::{
    runtime::{Builder, Runtime},
    sync::oneshot,
};

use blog::{result, server};

fn main() -> result::Result<()> {
    let mut runtime = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .thread_name("songday-web-service")
        .thread_stack_size(64 * 1024 * 1024)
        .build()?;

    let (tx, rx) = oneshot::channel::<()>();

    runtime.spawn(async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!("Shutting down web server...");
                let _ = tx.send(());
            },
            Err(e) => {
                eprintln!("{}", e);
            },
        }
    });

    println!("Starting web server...");
    let server = runtime.block_on(async { server::create_server("127.0.0.1:9270", rx).await.unwrap() });
    runtime.block_on(server);
    println!("Stopped web server...");

    Ok(())
    /*
    tokio::spawn(async move {
        let r = tokio::signal::ctrl_c().await;
        println!("ctrl-c received!");
    });
    println!("Starting web server...");
    server::start("127.0.0.1:9270").await
    */
}
