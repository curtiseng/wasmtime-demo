use clap::Parser;

/// Lightweight edge connect and compute runtime for various edge device!
///
/// Egccri is a edge program for connect devices, support modbus, mqtt, etc.
/// And it support compute with UDS(User define service).
#[derive(Parser, Debug)]
#[command(name = "Egccri")]
#[command(author = "Curtis Yang <zifeng.1024@gmail.com>")]
#[command(version = "1.0")]
#[command(about, long_about = None)]
struct Cli {
    /// Server proto
    server: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("server: {:?}", cli.server.as_deref());
}
