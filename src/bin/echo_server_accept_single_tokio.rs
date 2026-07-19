use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let (mut socket, _address) = listener.accept().await.unwrap();
    let (stream_reader, mut stream_writer) = socket.split();
    let mut message = String::new();
    let mut reader = BufReader::new(stream_reader);

    loop {
        reader.read_line(&mut message).await.unwrap();
        stream_writer.write_all(message.as_bytes()).await.unwrap();
        message.clear();
    }
}