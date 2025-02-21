mod cli;
mod file_io;
use cli::args::{CliArgs, Mycommand};
use cli::commands;
use file_io::reader;
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.command{
        Mycommand::Encrypt { file, key } => {
           
        }
        Mycommand::Decrypt { file, key } => {
            
        }
    }
}