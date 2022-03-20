use clap::{Args, Parser, Subcommand};

mod attack;
mod checkup;

#[derive(Debug, Parser)]
struct Cli {
    /// Command to run
    #[clap(subcommand)]
    command: Command,

    /// Debug mode
    #[clap(short, long)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Attack target
    Attack(Target),

    /// Check up on target
    Checkup(Target),
}

#[derive(Args, Debug)]
pub struct Target {
    /// Host to attack
    #[clap(long, default_value = "localhost")]
    host: String,

    /// Port on which to attack
    #[clap(short, long, default_value_t = 3000)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    if cli.debug {
        dbg!(&cli);
    }

    match &cli.command {
        Command::Attack(target) => {
            println!("Attack target {:?} on port {:?}", target.host, target.port);
            if !cli.debug {
                attack::send_attack(target);
            }
        }
        Command::Checkup(target) => {
            println!("Checkup target {:?} on port {:?}", target.host, target.port);
            if !cli.debug {
                checkup::send_checkup(target);
            }
        }
    }
}
