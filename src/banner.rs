const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const ORANGE: &str = "\x1b[38;5;208m";

const FERRIS: &str = r#"
      █ █         █ █
      ▀█  ▄█████▄  █▀
       ▀▄███▀█▀███▄▀
       ▄▀███▀▀▀███▀▄
       █ ▄▀▀▀▀▀▀▀▄ █
"#;

pub fn print(host: &str, port: u16) {
    let listen = format!("http://{host}:{port}");

    println!(
        "{ORANGE}{FERRIS}{RESET}
{CYAN}╭──────────────────────────────────────────╮{RESET}
{CYAN}│{RESET}  {BOLD}rust-user-service{RESET}                       {CYAN}│{RESET}
{CYAN}│{RESET}  {DIM}listening on{RESET} {BOLD}{listen:<26}{RESET} {CYAN}│{RESET}
{CYAN}│{RESET}  {DIM}health{RESET}        {CYAN}GET /health{RESET}               {CYAN}│{RESET}
{CYAN}╰──────────────────────────────────────────╯{RESET}
"
    );
}
