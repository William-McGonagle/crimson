use clap::{Args, Parser, Subcommand};
use std::env;

mod help;
mod helper;
mod checkup;
mod attack;

fn attack_argument(args:&[String]) {

    if args.len() < 2 { println!("Target needed."); return; }

    let target = &args[1];
    let split_target = helper::split_port_and_address(target).unwrap();

    println!("Starting Attack on {} with {} Workers...", "localhost:3000", 300);

    attack::send_attack(split_target[0], split_target[1]);

}

fn checkup_argument(args:&[String]) {

    if args.len() < 2 { println!("Target needed."); return; }

    let target = &args[1];
    let split_target = helper::split_port_and_address(target).unwrap();

    println!("Starting Checkup on Server at {}:{}", split_target[0], split_target[1]);

    checkup::send_checkup(split_target[0], split_target[1]);

}

fn not_found_argument(args:&[String]) {

    let main_argument = &args[0]; // to upper

    const OPTIONS:[&str;3] = [
        "help",
        "attack",
        "checkup", // test the http-server as a whole
    ];

    let mut min_distance = 100000;
    let mut min_distance_option = "help";

    for option in OPTIONS {

        // find the distances
        let length_difference = (option.len() as i32) - (main_argument.len() as i32);
        let cur_distance = if length_difference > 0 { length_difference } else { -length_difference };

        // if distance is smaller than current distance, set it as new 
        if cur_distance < min_distance { 
            
            min_distance = cur_distance; 
            min_distance_option = option;
        
        }

    }

    println!("Did you mean \"{}\"?", min_distance_option);

}

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
struct Target {
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
            println!("Attack target {:?} on port {:?}", target.host, target.port)
        }
        Command::Checkup(target) => {
            println!("Checkup target {:?} on port {:?}", target.host, target.port)
        }
    }
}
