use owo_colors::OwoColorize;

pub enum Status {
    Ok,
    Warn,
    Error,
}

pub fn log_boot(status: Status, system: &str, info: &str) {
    match status {
        Status::Ok => {
            println!("{}", ("[ OK ] ".to_owned() + system + " - " + info).green().on_black());
        }
        Status::Warn => {
            println!("{}", ("[ WARN ] ".to_owned() + system + " - " + info).yellow().on_black());
        }
        Status::Error => {
            println!("{}", ("[ FAIL ] ".to_owned() + system + " - " + info).red().on_black());
        }
    }
}