use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::select;
use tokio::signal;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    let token = CancellationToken::new();
    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let token = token.clone();
        let cancelled = token.clone();
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(_) => {
                    println!("Ctrl+C");
                    cancelled.cancel();
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        });
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut message = String::new();
            let mut reader_buf = BufReader::new(reader);
            loop {
                select! {
                    result = reader_buf.read_line(&mut message) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((message.clone(),_address)).unwrap();
                        println!("{:?} SEND:{}", _address, message);
                        message.clear();
                    }
                    result = rx.recv() => {
                        let (msg,addr) = result.unwrap();
                        if addr != _address {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                    _ = token.cancelled() => {
                        println!("{:?} CLOSE Clean up...", _address);
                        break;
                    }
                }
            }
        });
    }
}
