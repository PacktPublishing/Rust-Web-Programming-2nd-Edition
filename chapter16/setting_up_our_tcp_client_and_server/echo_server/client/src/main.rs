use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.split();

    println!("stream starting");
    writer.write_all(b"this is a test\n").await?;
    println!("sent data");

    let mut buf_reader = BufReader::new(reader);
    let mut buf = vec![];
    println!("reading data");
    let _ = buf_reader.read_until(b'\n', &mut buf).await.unwrap();
    let message = String::from_utf8_lossy(&buf);
    println!("{}", message);
    Ok(())
}
