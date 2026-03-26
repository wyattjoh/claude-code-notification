use anyhow::Result;
use clap::{Arg, Command};
use claude_code_notification::{main as notification_main, Sound};
use std::io;

mod setup;

fn main() -> Result<()> {
    let matches = Command::new("claude-code-notification")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Claude Code hook for displaying desktop notifications")
        .arg(
            Arg::new("sound")
                .long("sound")
                .value_name("SOUND_NAME")
                .help("System sound to play with notification")
                .default_value("Glass"),
        )
        .arg(
            Arg::new("activate-terminal")
                .long("activate-terminal")
                .action(clap::ArgAction::SetTrue)
                .help("Automatically activate the terminal application when notification is displayed"),
        )
        .subcommand(Command::new("setup").about("Configure Claude Code settings for notifications"))
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => setup::run_setup(),
        _ => {
            let sound_name = matches.get_one::<String>("sound").unwrap();
            let sound = Sound::from_name(sound_name);

            let activate_terminal = matches.get_flag("activate-terminal");

            let stdin = io::stdin();
            notification_main(stdin, sound, activate_terminal)
        }
    }
}
