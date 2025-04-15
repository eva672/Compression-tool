use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

mod lz;
mod rle;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file
    Compress {
        /// Input file path
        input: PathBuf,
        /// Output file path
        output: PathBuf,
        /// Use RLE compression
        #[arg(short, long)]
        rle: bool,
        /// Use LZ77 compression
        #[arg(short, long)]
        lz: bool,
    },
    /// Decompress a file
    Decompress {
        /// Input file path
        input: PathBuf,
        /// Output file path
        output: PathBuf,
        /// Use RLE decompression
        #[arg(short, long)]
        rle: bool,
        /// Use LZ77 decompression
        #[arg(short, long)]
        lz: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress {
            input,
            output,
            rle,
            lz,
        } => {
            let input_file = File::open(input)?;
            let output_file = File::create(output)?;

            let reader = BufReader::new(input_file);
            let writer = BufWriter::new(output_file);

            if rle {
                rle::compress(reader, writer)?;
            } else if lz {
                lz::compress(reader, writer)?;
            } else {
                eprintln!("Please specify compression algorithm (--rle or --lz)");
                std::process::exit(1);
            }
        }
        Commands::Decompress {
            input,
            output,
            rle,
            lz,
        } => {
            let input_file = File::open(input)?;
            let output_file = File::create(output)?;

            let reader = BufReader::new(input_file);
            let writer = BufWriter::new(output_file);

            if rle {
                rle::decompress(reader, writer)?;
            } else if lz {
                lz::decompress(reader, writer)?;
            } else {
                eprintln!("Please specify decompression algorithm (--rle or --lz)");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
