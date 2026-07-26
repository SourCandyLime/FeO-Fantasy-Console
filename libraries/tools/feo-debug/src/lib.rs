use owo_colors::OwoColorize;

pub enum Status {
    Ok,
    Warn,
    Error,
}

pub fn log_boot(status: Status, system: &str) {
    match status {
        Status::Ok => {
            println!("{}{}", "[ OK ] : ".green().on_black(), system.green().on_black());
        }
        Status::Warn => {
            println!("{}{}", "[WARN] : ".yellow().on_black(), system.yellow().on_black());
        }
        Status::Error => {
            println!("{}{}", "[FAIL] : ".red().on_black(), system.red().on_black());
        }
    }
}