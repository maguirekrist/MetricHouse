use std::{io::Write, net::TcpStream, time::Instant};

use lib::{models::Metric, traits::serializable::BinarySerializable};


const BIND_ADDRESS: &str = "127.0.0.1:1227";

fn send_trace(trace: &Metric) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(BIND_ADDRESS)?;

    let serialized_trace = trace.serialize(); 
    let mut buf= Vec::with_capacity(1 + serialized_trace.len());
    buf[0] = 1; // Control byte for write operation
    buf[1..].copy_from_slice(&serialized_trace);
    stream.write_all(&serialized_trace)?;
    Ok(())
}

fn main() {
    println!("Hello, world!");

    let test = Metric {
        timestamp: time::OffsetDateTime::now_utc().unix_timestamp() as u64,
        name: "test_metric".to_owned(),
        labels: vec![("test_label".to_owned(), "test_value".to_owned())]
    };

    let start = Instant::now();

    let total_metrics = 100_000;
    for _ in 0..100 {
        send_trace(&test).expect("Failed to send trace");
    }

    let duration = start.elapsed();
    let seconds = duration.as_secs_f64();
    println!("Sent {} metrics in {:.2} sec -> {:.2} metrics/sec", total_metrics, seconds, total_metrics as f64 / seconds);
}
