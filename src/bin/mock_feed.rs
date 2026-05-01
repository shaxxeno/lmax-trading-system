use std::io::Write;
use std::net::TcpStream;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    println!("Mock MT5 feed connecting to 127.0.0.1:9000...");
    let mut stream = TcpStream::connect("127.0.0.1:9000").expect("Failed to connect");
    println!("Connected. Sending ticks...");

    let mut price = 1.0850_f64;
    let mut buf = [0u8; 16];
    let mut i = 0u64;

    loop {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;

        // simulate small price movement
        price += match i % 7 {
            0 => 0.0001,
            3 => -0.0001,
            _ => 0.0,
        };

        buf[0..8].copy_from_slice(&price.to_le_bytes());
        buf[8..16].copy_from_slice(&timestamp.to_le_bytes());

        if stream.write_all(&buf).is_err() {
            println!("Connection lost");
            break;
        }

        i += 1;
        std::thread::sleep(Duration::from_millis(1));
    }
}
