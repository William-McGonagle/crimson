pub fn help_argument(args:&[String]) {

    if args.len() == 1 {

        // General Help
        println!("You can use the following commands...");
        println!(" - help");
        println!(" - attack");
        println!(" - checkup");

    }

    if args.len() == 2 {

        // Help on Specific Topic
        let main_argument:&str = &args[1];
        println!("{}", match main_argument {

            "help" => "The help command gives you info on commands.",
            "attack" => "The attack command sends a swarm to attack a server.",
            "checkup" => "The checkup command performs an entire test suite on a server.",
            _ => "I can't find that command..."

        })

    }

    return;

}