extern crate reqwest;
extern crate json;
mod file;
use self::file::HastebinFile;
use std::fs;
use std::io::Read;
use reqwest::Client;


/**
 * Upload the buffer to hastebin
 */
pub fn upload_buffer(file_name: &str, max_chars: Option<&str>) {
    // Checking for any errors while reading the file
    match read_file(file_name) {
        Ok(hastebin_file) => {
            // Checking if the file is too big
            if hastebin_file.is_too_big() {
                println!("Error: File is too big, 50MB max.");
                std::process::exit(0);
            } else {
                // upload the file to hastebin
                let client = Client::new();
                let mut response = client.post("https://hastebin.com/documents")
                    .body(hastebin_file.get_buffer())
                    .send()
                    .expect("Cannot send the response");
                // parse the response body
                match json::parse(&response.text().unwrap()) {
                    Ok(res_json) => println!("File uploaded successfully: https://hastebin.com/{}", res_json["key"].to_string()),
                    Err(e) => println!("Failed to parse the body: {}", e)
                }
                
            }
        },
        Err(e) => println!("An error occurred while reading the file: {}", e)
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