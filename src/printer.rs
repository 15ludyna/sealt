use colored::Colorize;

pub fn print_success_msg(msg: &str) {
    println!(
        "{}{}", 
        "[+]".green(),
        msg
    );
}

pub fn print_error_msg(msg: &str) {
    println!(
        "{}{}", 
        "[-]".red(),
        msg
    );
}

pub fn print_thanks_msg(msg: &str) {
    println!(
        "{}{}", 
        "[❤️]".red(),
        msg
    );
}
