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

    /// 🔒 파일 암호화: 지정된 알고리즘(AES 또는 ChaCha)으로 파일을 암호화합니다.
    Encrypt{
        #[arg(short,long)]
        file : String,

        #[arg(short,long)]
        key: String,

        #[arg(short,long, default_value = "aes")]
        algo: String,

    },

    /// 🔓 파일 복호화: 같은 알고리즘(AES 또는 ChaCha)으로 암호화된 파일을 복호화합니다.
    Decrypt{
        #[arg(short,long)]
        file: String,

        #[arg(short,long)]
        key:String,

        #[arg(short,long, default_value = "aes")]
        algo: String,

    }
}

