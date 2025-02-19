mod cli;
use cli::args::{CliArgs, Mycommand};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.command{
        Mycommand::Encrypt { file, key } => {
            println!("{:?}, {:?}",file,key);
        }
        Mycommand::Decrypt { file, key } => {
            println!("{:?}, {:?}",file,key);
        }
    }
}