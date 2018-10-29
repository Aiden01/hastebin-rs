extern crate json;
extern crate reqwest;
mod file;
use self::file::HastebinFile;
use reqwest::Client;
use std::fs;
use std::io::Read;

/**
 * Upload the buffer to hastebin
 */
pub fn upload_buffer<'a>(
    file_name: &'a str,
    max_chars: Option<&'a str>,
) -> std::io::Result<String> {
    let hastebin_file = read_file(file_name)?;

    if hastebin_file.is_too_big() {
        println!("Error: File is too big, 50MB max.");
        std::process::exit(0);
    } else {
        // upload the file to hastebin
        let client = Client::new();
        let mut response = client
            .post("https://hastebin.com/documents")
            .body(hastebin_file.get_buffer())
            .send()
            .expect("Cannot send the response");

        // parse the response body
        let res_json = json::parse(&response.text().unwrap()).unwrap();
        let url = format!("https://hastebin.com/{}", res_json["key"].to_string());
        Ok(url)
    }
}

/**
 * Read the file and returns HastebinFile object
 */
fn read_file(path: &str) -> std::io::Result<HastebinFile> {
    // open the file
    let mut file: fs::File = fs::File::open(path)?;
    let mut buffer = String::new();
    // read the file
    file.read_to_string(&mut buffer)?;
    // return HastebinFile object with the informations of the file
    Ok(HastebinFile::new(buffer, String::from(path)))
}
