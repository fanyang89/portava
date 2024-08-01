mod cli;

use crate::cli::Commands;
use anyhow::{Ok, Result};
use clap::Parser;
use number_range::{NumberRange, NumberRangeOptions};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use tokio::time::Instant;
use tracing::{info, Level};

fn port_range_option() -> NumberRangeOptions<u16> {
    NumberRangeOptions::<u16> {
        list_sep: ',',
        range_sep: '-',
        decimal_sep: '.',
        group_sep: '_',
        whitespace: false,
        default_start: None,
        default_end: None,
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let verbose = cli.verbose;
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(if verbose { Level::TRACE } else { Level::INFO })
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .init();

    match &cli.command {
        Commands::Listen {
            ip_addr,
            port_range,
        } => {
            let ports = port_range_option().parse(port_range.as_str())?;
            let mut bind_success_ports = vec![];
            let mut accept_success_ports = vec![];
            let mut failed_ports = vec![];
            let mut conns = vec![];

            for port in ports {
                match TcpListener::bind(SocketAddr::new(ip_addr.clone(), port)).await {
                    Result::Ok(conn) => {
                        conns.push((port, conn));
                        bind_success_ports.push(port);
                    }
                    Err(_) => {
                        failed_ports.push(port);
                    }
                }
            }

            let success_port_range = NumberRange::<u16>::from_options(port_range_option())
                .from_vec(bind_success_ports, None);
            let failed_port_range =
                NumberRange::<u16>::from_options(port_range_option()).from_vec(failed_ports, None);
            info!("Bind success ports: {}", success_port_range.to_string());
            info!("Bind failed ports: {}", failed_port_range.to_string());

            for (port, conn) in conns.iter() {
                let _ = conn.accept().await;
                accept_success_ports.push(*port);
            }
            let accepted_port_range = NumberRange::<u16>::from_options(port_range_option())
                .from_vec(accept_success_ports, None);
            info!("Accept success ports: {}", accepted_port_range.to_string());
        }

        Commands::Connect {
            ip_addr,
            port_range,
            timeout_ms,
        } => {
            let ports = port_range_option().parse(port_range.as_str())?;
            let timeout = Duration::from_millis(*timeout_ms);
            let mut success_ports = vec![];
            let mut failed_ports = vec![];

            let mut total = 0;
            let now = Instant::now();
            for port in ports {
                let ok = match time::timeout(
                    timeout,
                    TcpStream::connect(SocketAddr::new(ip_addr.clone(), port)),
                )
                .await
                {
                    Result::Ok(conn) => match conn {
                        Result::Ok(_) => true,
                        Err(_) => false,
                    },
                    Err(_) => false,
                };
                if ok {
                    success_ports.push(port);
                } else {
                    failed_ports.push(port);
                }
                total += 1;
            }
            let elapsed = Instant::now() - now;
            info!("Scan {} ports in {}s", total, elapsed.as_secs());

            let success_port_range =
                NumberRange::<u16>::from_options(port_range_option()).from_vec(success_ports, None);
            let failed_port_range =
                NumberRange::<u16>::from_options(port_range_option()).from_vec(failed_ports, None);
            info!("Connect success ports: {}", success_port_range.to_string());
            info!("Connect failed ports: {}", failed_port_range.to_string());
        }
    }

    Ok(())
}
