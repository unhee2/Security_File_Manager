// 명령 인수 관리 모듈 -> CLI 라이브러리 clap 사용
use clap::{Parser,Subcommand};

#[derive(Parser,Debug)]
#[command(name = "Secure File Manager")]
#[command(version = "1")]
#[command(about = "AES-256 & ChaCha20 기반 파일 암호화 도구", long_about = None)]
pub struct CliArgs{ 
    #[command(subcommand)]
    pub command : Mycommand,
}

#[derive(Subcommand,Debug)]
pub enum Mycommand{

    /// 파일 암호화 : cargo run -- encrypt --file <file.txt> --key <key>
    Encrypt{
        #[arg(short,long)]
        file : String,

        #[arg(short,long)]
        key: String,
    },

    /// 파일 복호화 : cargo run -- decrypt --file <file.txt> --key <key>
    Decrypt{
        #[arg(short,long)]
        file: String,

        #[arg(short,long)]
        key:String,
    }
}

