use clipboard_manager::ClipboardManager;
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
    time::sleep,
};

pub mod clipboard_manager;
pub mod error;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8000")
        .await
        .expect("==> Port unavailable...");

    let clipboard = Arc::new(Mutex::new(ClipboardManager::new()));

    let clipboard1 = clipboard.clone();
    let clipboard2 = clipboard.clone();

    tokio::join!(run_local(clipboard1), run_server(listener, clipboard2),);
}

async fn run_server(listener: TcpListener, clipboard: Arc<Mutex<ClipboardManager>>) {
    let mut buf = [0_u8; 10000];

    while let Ok((mut stream, _)) = listener.accept().await {
        if let Ok(ln) = stream.read(&mut buf).await {
            println!("{}", ln);
            clipboard
                .lock()
                .await
                .update(String::from_utf8_lossy(&buf[..ln]).to_string());
        }
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
            .await
            .unwrap();
    }
}
async fn run_local(clipboard: Arc<Mutex<ClipboardManager>>) {
    loop {
        clipboard.lock().await.get_and_update();
        println!("{}", clipboard.lock().await.get());
        sleep(Duration::from_secs(2)).await;
    }
}
