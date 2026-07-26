mod commands;

use commands::*;

use std::io::{self, Write};

fn console() {
    let commands = commands::all();
    loop {
        print!("feo> ");

        // Make sure the prompt appears immediately.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let name = parts[0];
        let args = &parts[1..];

        match commands.iter().find(|c| c.name() == name) {
            Some(command) => match command.execute(args) {
                CommandResult::Continue => {}
                CommandResult::Exit => break,
            },
            None => println!("Unknown command '{}'. Type 'help' for a list of commands.", name),
        }
    }
}

fn main() {
    println!("FeO Fantasy Console V0.0.1");
    println!("  Operating System: {}", feo_system::os());
    println!("  Architecture: {}", feo_system::arch());
    println!("\nBooting...\n");


    let SYS = feo_system::init();
    feo_clock::init();
    feo_memory::init();
    feo_math::init();

    feo_asset::init();
    feo_file::init();
    feo_plug::init();

    feo_color::init();
    feo_render::init();
    feo_gui::init();
    

    feo_input::init();
    feo_audio::init();
    feo_physics::init();

    feo_sdk::init();

    println!("\nFinalizing...");
    println!("\nREADY\n");
    console();
}
