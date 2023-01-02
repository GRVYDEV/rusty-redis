use clap::Parser;
use tokio::net::TcpListener;

/// A simple redis server written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to start the server on
    #[arg(short, long, default_value_t = 6379)]
    port: i32,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    println!("server starting on port {}", args.port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted connection {:?}", socket);
    }
}
