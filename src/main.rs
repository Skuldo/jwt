use clap::{Parser, Subcommand, Args};
//use crate::base64::base64_pad;
use crate::jwt::jwt_decode;

mod base64;
mod jwt;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///Output formatted
    #[clap(short, long, global = true)]
    fmt: Option<String>,
    
    ///Output formatted & colourised
    #[clap(short, long, global = true)]
    colour: Option<String>,

    ///Output raw
    #[clap(short, long, global = true)]
    raw: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// JSON Web Token header section
    Header {jwt: String},
    /// JSON Web Token payload section
    Payload {jwt: String},
    /// JSON Web Token signature section
    Signature {jwt: String},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Header { jwt } => print!("{}", jwt_decode(jwt.to_string(), jwt::Sections::Header)),
        Commands::Payload { jwt } => print!("{}", jwt_decode(jwt.to_string(), jwt::Sections::Payload)),
        Commands::Signature { jwt } => print!("{}", jwt_decode(jwt.to_string(), jwt::Sections::Signature)),
    }

    /*
    match cli {
        Some(jwt) => todo!(),
    }
    */

    //println!("{}", jwt_decode(cli, jwt::Sections::All));
}
