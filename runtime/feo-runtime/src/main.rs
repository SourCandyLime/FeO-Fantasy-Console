use feo_debug::{log_boot, Status};

fn main() {
    println!("FeO Fantasy Console V0.0.1\n");
    println!("Booting...\n");


    log_boot(Status::Ok, "Math");
    log_boot(Status::Ok, "Memory");
    log_boot(Status::Ok, "Time");
    log_boot(Status::Warn, "System");
    log_boot(Status::Ok, "Asset");
    log_boot(Status::Warn, "Plug");
    log_boot(Status::Error, "Render");
    log_boot(Status::Ok, "Input");
    log_boot(Status::Warn, "Color");
    log_boot(Status::Warn, "Audio");
    log_boot(Status::Warn, "File");
    log_boot(Status::Warn, "GUI");
    log_boot(Status::Warn, "Physics");
    log_boot(Status::Ok, "SDK");

    println!("\nFinalizing...")
}
