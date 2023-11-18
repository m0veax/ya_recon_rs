use std::env;

// Reading arguments to determin what to do

struct EnvArgs{
    selected_module: String,
    selected_mode: String,
    target: String
}

/**
TODO use clap crate

return EnvArgs
**/
fn read_args() -> EnvArgs {
    // Get the command-line arguments.
    let args: Vec<String> = env::args().collect();

    // print while using Debug trait
    println!("{:?}", args);

    // Check if at least 4 arguments are provided (the first argument is the program name).
    if args.len() < 4 {
        eprintln!("Usage: {} <module> <mode> <target>", args[0]);
        std::process::exit(1);
    }

    // Get the arguments
    let selected_mode = args[1].clone();
    let selected_module = args[2].clone();
    let target = args[3].clone();

    // Now, you can work with the `selected_module` and `target` as needed.
    println!("The first argument is: {}", selected_module);

    // Create an instance of the `EnvArgs` struct and return it.
    EnvArgs {
        selected_module,
        selected_mode,
        target,
    }
}


fn whitelist_modes(mode: &str) -> bool {
    let modes = vec!["nmap"];
    
    modes.contains(&mode)
    
}


fn main() {
    
    println!("Start ...");
    let args = read_args();

    println!( "Selected Module: {}", args.selected_module);
    println!( "Selected Mode: {}", args.selected_mode);
    println!( "Target: {}", args.target);

    if whitelist_modes(&args.selected_mode) {
        println!("Found Mode: {}", args.selected_mode);
    }
}