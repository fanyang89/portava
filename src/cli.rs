use std::net::IpAddr;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Try to listen at range
    Listen {
        #[arg(short, long, required = true, default_value = "0.0.0.0")]
        ip_addr: IpAddr,

        #[arg(short, long, required = true)]
        port_range: String,
    },

    /// Try to connect to range
    Connect {
        #[arg(short, long, required = true)]
        ip_addr: IpAddr,

        #[arg(short, long, required = true)]
        port_range: String,

        #[arg(short, long, default_value_t = 3000u64)]
        timeout_ms: u64,
    },
}
