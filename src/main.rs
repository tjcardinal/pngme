#![allow(dead_code)]
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Args::parse();
    match args {
        args::Args::Encode {
            filename,
            chunk_type,
            msg,
            output,
        } => commands::encode(filename, chunk_type, msg, output),
        args::Args::Decode {
            filename,
            chunk_type,
        } => commands::decode(filename, chunk_type),
        args::Args::Print { filename } => commands::print(filename),
        args::Args::Remove {
            filename,
            chunk_type,
        } => commands::remove(filename, chunk_type),
    }
}
