use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "unitconv")]
#[command(version = "0.1.0")]
#[command(about = "Aplikasi konversi satuan suhu, panjang, dan berat", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Convert {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
    List,
    History,
}
