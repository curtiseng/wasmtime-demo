use futures::future;
use std::{net::SocketAddr, time::Duration};
use btleplug::api::{Central, Manager as _, ScanFilter, Peripheral};
use btleplug::platform::Manager;
use tokio::time;

use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    tokio::select! {
        _ = server_context(socket_addr) => unreachable!(),
        _ = client_context(socket_addr) => println!("Client exiting"),
    }

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }
    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                println!(
                    "Peripheral {:?} is connected: {:?}",
                    local_name, is_connected
                );
                if !is_connected {
                    println!("Connecting to peripheral {:?}...", &local_name);
                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {:?}", err);
                        continue;
                    }
                }
                let is_connected = peripheral.is_connected().await?;
                println!(
                    "Now connected ({:?}) to peripheral {:?}...",
                    is_connected, &local_name
                );
                peripheral.discover_services().await?;
                println!("Discover peripheral {:?} services...", &local_name);
                for service in peripheral.services() {
                    println!(
                        "Service UUID {}, primary: {}",
                        service.uuid, service.primary
                    );
                    for characteristic in service.characteristics {
                        println!("  {:?}", characteristic);
                    }
                }
                if is_connected {
                    println!("Disconnecting from peripheral {:?}...", &local_name);
                    peripheral
                        .disconnect()
                        .await
                        .expect("Error disconnecting from BLE peripheral");
                }
            }
        }
    }
    Ok(())
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
            println!("Client: The read_input_registers result is '{:?}'", response);

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
