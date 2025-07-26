use std::sync::{Arc, RwLock};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use lib::{db::MetricsDb, models::Metric, traits::serializable::BinarySerializable};

const BIND_ADDRESS: &str = "127.0.0.1:1227";

async fn handle_client(stream: TcpStream, db: &Arc<RwLock<MetricsDb>>) -> tokio::io::Result<()> {

    stream.readable().await?;
    let mut buf_reader = BufReader::new(stream);
    let data = buf_reader.fill_buf().await?;

    let control_byte = data[0];
    match control_byte {
        0 => handle_read(data, db).await,
        1 => handle_write(data, db).await,
        _ => {
            eprintln!("Unknown control byte: {}", control_byte);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unknown control byte"));
        }
    }?;

    Ok(())
}

async fn handle_read(data: &[u8], db: &Arc<RwLock<MetricsDb>>) -> tokio::io::Result<()> {
    let content = &data[1..];
    let len = u32::from_le_bytes(content[0..4].try_into().unwrap()) as usize;
    let name = String::from_utf8(content[4..4 + len].to_vec()).unwrap();

    let guard = db.read().unwrap();
    let _metrics = guard.query(&name);

    //TODO: I guess return the results back to the client??
    Ok(())
}

async fn handle_write(data: &[u8], db: &Arc<RwLock<MetricsDb>>) -> std::io::Result<()> {
    let content = &data[1..];
    let mut byte_offset: usize = 0;
    let metric = Metric::deserialize(content, &mut byte_offset).unwrap();

    let mut guard = db.write().unwrap();
    guard.ingest(metric);

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(BIND_ADDRESS).await
        .expect(&format!("Failed to bind to address {}", BIND_ADDRESS)); 

    //let mut wal_writer = WalWriter::new();    
    let db = Arc::new(RwLock::new(MetricsDb::new()));
    //let arena = Arena::new(1024 * 1024); // 1MB capacity
    println!("Server is listening on {}", BIND_ADDRESS);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        tokio::spawn({
            let db = db.clone();
            async move {
                handle_client(socket, &db).await.unwrap();
            }
        });
    }

    Ok(())
}
