pub fn print_red(message: &str) {
    println!("\x1b[31;1mwatchdog_rs | {}\x1b[0m", message);
}

pub fn print_green(message: &str) {
    println!("\x1b[32;1mwatchdog_rs | {}\x1b[0m", message);
}

pub fn print_yellow(message: &str) {
    println!("\x1b[33;1mwatchdog_rs | {}\x1b[0m", message);
}

pub fn print_blue(message: &str) {
    println!("\x1b[34;1mwatchdog_rs | {}\x1b[0m", message);
}