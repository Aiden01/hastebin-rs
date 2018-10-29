extern crate clap;
extern crate hastebin;

use clap::{App, Arg, ArgMatches};
use hastebin::Os;
use std::process::Command;

/**
 * Build the cli
 */
fn build_cli() -> App<'static, 'static> {
    App::new("hastebin")
        .version("1.0.0")
        .author("WebD")
        .arg(
            Arg::with_name("FILE")
                .help("The file that you want to send to hastebin")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("CHARS")
                .help("Maximun characters to send")
                .short("c")
                .long("chars")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OPEN")
                .help("Opens your web browser when it's uploaded.")
                .short("o")
                .long("open"),
        )
}

/**
 * Returns the operating system of the user
*/
fn get_os() -> Os {
    if cfg!(target_os = "linux") {
        Os::Linux
    } else if cfg!(target_os = "macos") {
        Os::Macos
    } else if cfg!(target_os = "windows") {
        Os::Window
    } else {
        Os::Unknow
    }
}

/**
 * Opens the user's browser when the upload is finished
*/
fn open_browser(url: String) {
    let win = &format!("/C {}", url);

    let (cmd, arg): (&str, &str) = match get_os() {
        Os::Window => ("cmd", win),
        Os::Macos => ("open", &url),
        Os::Linux => ("xdg-open", &url),
        Os::Unknow => panic!("Unknow OS, cannot open your browser"),
    };

    Command::new(cmd)
        .arg(arg)
        .output()
        .expect("Cannot open your browser");
}

fn main() {
    // Get the arguments
    let args: ArgMatches = build_cli().get_matches();

    if let Some(file_name) = args.value_of("FILE") {
        println!("Uploading file...");

        match hastebin::upload_buffer(file_name, args.value_of("CHARS")) {
            Ok(url) => {
                println!("File uploaded successfully: {}", url);
                if args.is_present("OPEN") {
                    open_browser(url);
                }
            }
            Err(e) => println!("An error occurred: {}", e),
        };
    }
}
