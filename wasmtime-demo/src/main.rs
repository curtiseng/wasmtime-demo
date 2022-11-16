use futures::future;
use std::{net::SocketAddr, time::Duration};

use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};
use tracing::info;
use tracing_subscriber;

mod ble;

pub struct Demo;

impl Demo {
    pub fn log(&self) {
        info!("Demo log")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let socket_addr = "127.0.0.1:5502".parse().unwrap();
    //
    // tokio::select! {
    //     _ = server_context(socket_addr) => unreachable!(),
    //     _ = client_context(socket_addr) => println!("Client exiting"),
    // }

    tracing_subscriber::fmt::init();

    Demo.log();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    tracing_lib::test();
    // let number_shaved = yak_shave::shave_all(number_of_yaks);
    // info!(
    //     all_yaks_shaved = number_shaved == number_of_yaks,
    //     "yak shaving completed."
    // );

    Ok(())
}

struct MbServer;

impl Service for MbServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(_addr, cnt) => {
                let mut registers = vec![0; cnt.into()];
                println!("Server: Set third register 77");
                registers[2] = 77;
                future::ready(Ok(Response::ReadInputRegisters(registers)))
            }
            Request::WriteMultipleRegisters(_addr, cnt) => {
                println!("Server: The request is '{:?}'", cnt);
                future::ready(Ok(Response::WriteMultipleRegisters(0x00, 7)))
            }
            _ => unimplemented!(),
        }
    }
}

async fn server_context(socket_addr: SocketAddr) {
    println!("Server: starting up server...");
    let server = server::tcp::Server::new(socket_addr);
    server.serve(|| Ok(MbServer)).await.unwrap();
}

async fn client_context(socket_addr: SocketAddr) {
    tokio::join!(
        async {
            // Give the server some time for starting up
            tokio::time::sleep(Duration::from_secs(1)).await;

            println!("Client: Connecting client...");
            let mut ctx = tcp::connect(socket_addr).await.unwrap();
            println!("Client: Reading input registers...");
            let response = ctx.read_input_registers(0x00, 7).await.unwrap();
            println!(
                "Client: The read_input_registers result is '{:?}'",
                response
            );

            println!("Client: Writing multiple registers...");
            let word = [0, 0, 88, 0, 0, 0, 0];
            let response = ctx.write_multiple_registers(0x00, &word).await;
            match response {
                Err(e) => println!("Write error: '{:?}'", e),
                _ => println!("Client: Write success, well done..."),
            }
        },
        tokio::time::sleep(Duration::from_secs(5))
    );
}
