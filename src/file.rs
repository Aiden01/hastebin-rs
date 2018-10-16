
use std::fs::{
    Metadata,
    metadata
};

#[derive(Debug)]
pub struct HastebinFile {
    buffer: String,
    path: String
}

fn get_max_bytes_upload() -> u64 {
    52428800
}

impl HastebinFile {

    /**
     * Creates a new instance of the struct
     */
    pub fn new(buffer: String, path: String) -> HastebinFile {
        HastebinFile{
            buffer: buffer,
            path: path
        }
    }

    /**
     * Check if the file is too big
     */
    pub fn is_too_big(&self) -> bool {
        // Get the file's metadatas
        let file_metadata: Metadata = metadata(self.get_path()).expect("Cannot get file metadata");
        file_metadata.len() >= get_max_bytes_upload()
    }

    /**
     * Returns the file's buffer
     */
    pub fn get_buffer(&self) -> String {
        self.buffer.clone()
    }

    /**
     * Returns the file's path
     */
    pub fn get_path(&self) -> &str {
        &self.path
    }

}