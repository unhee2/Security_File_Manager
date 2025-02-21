mod cli;
mod file_io;
mod encryption;
use cli::args::{CliArgs, Mycommand};
use cli::commands;
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.command{
        Mycommand::Encrypt { file, key, algo } => {
            println!("🔒 파일 암호화 시작: {}", file);
            if let Err(e) = commands::encrypt_file(&file, &key, &algo) {
                eprintln!("❌ 암호화 실패: {}", e);
            }
        }
        Mycommand::Decrypt { file, key, algo } => {
            println!("🔓 파일 복호화 시작: {}", file);
            if let Err(e) = commands::decrypt_file(&file, &key, &algo) {
                eprintln!("❌ 복호화 실패: {}", e);
            }
        }
    }
}