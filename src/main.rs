use std::env;
use std::time::Duration;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }

    let port = &args[1];
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let filename = format!("{}.txt", port);

    loop {
        let (socket, _) = listener.accept().await?;
        let filename = filename.clone();

        tokio::spawn(async move {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(&filename)
                .await
                .expect("Failed to open file");

            handle_client(&mut file, socket)
                .await
                .expect("Error handling client");
        });
    }
}

async fn handle_client(
    file: &mut tokio::fs::File,
    mut socket: tokio::net::TcpStream,
) -> std::io::Result<()> {
    let mut buffer = [0; 128];
    let n = socket.read(&mut buffer).await?;
    if n > 0 {
        file.write_all(&buffer[0..n]).await?;
        tokio::time::sleep(Duration::from_secs(3)).await;
        socket.write_all(b"ACCEPTED").await?;
    }
    Ok(())
}
