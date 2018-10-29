extern crate clap;
extern crate hastebin;

use clap::{App, Arg, ArgMatches};

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
 * Opens the user's browser when the upload is finished
*/
fn open_browser(url: &str) {}

fn main() {
    // Get the arguments
    let args: ArgMatches = build_cli().get_matches();

    if let Some(file_name) = args.value_of("FILE") {
        println!("Uploading file...");

        match hastebin::upload_buffer(file_name, args.value_of("CHARS")) {
            Ok(url) => {
                println!("File uploaded successfully: {}", url);
                if args.is_present("OPEN") {}
            }
            Err(e) => println!("An error occurred: {}", e),
        };
    }
}
