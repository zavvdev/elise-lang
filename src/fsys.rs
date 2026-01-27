pub mod file_reader {
    use std::fs;

    pub struct FileReaderDescriptor {
        pub path: String,
        pub content: String,
    }

    pub struct FileReaderError {
        pub message: String,
    }

    pub fn read_file(path: &str) -> Result<FileReaderDescriptor, FileReaderError> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(FileReaderDescriptor {
                path: path.to_string(),
                content,
            }),
            Err(e) => Err(FileReaderError {
                message: format!("Failed to read file: {}", e),
            }),
        }
    }
}
