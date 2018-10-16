
extern crate clap;
extern crate hastebin;

use clap::{
    App,
    Arg,
    ArgMatches
};

/**
 * Build the cli
 */
fn build_cli() -> App<'static, 'static> {
    App::new("hastebin")
        .version("1.0.0")
        .author("WebD")
        .arg(Arg::with_name("FILE")
            .help("The file that you want to send to hastebin")
            .required(true)
            .index(1))
        .arg(Arg::with_name("CHARS")
            .help("Maximun characters to send")
            .short("c")
            .long("chars")
            .takes_value(true))
}

fn main() {
    // Get the arguments
    let args: ArgMatches = build_cli().get_matches();
    if let Some(file_name) = args.value_of("FILE") {
       hastebin::upload_buffer(file_name, args.value_of("CHARS"));
    } 
}