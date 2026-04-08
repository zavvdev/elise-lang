use std::fs;

#[derive(PartialEq, Debug)]
pub struct FileDescriptor {
    pub path: String,
    pub content: String,
}

#[derive(PartialEq, Debug)]
pub struct FileReaderError {
    pub message: String,
    pub path: String,
}

#[derive(PartialEq, Debug)]
pub struct FileWriterError {
    pub message: String,
}

fn read_file(path: &str) -> Result<FileDescriptor, FileReaderError> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(FileDescriptor {
            path: path.to_string(),
            content,
        }),
        Err(e) => Err(FileReaderError {
            message: e.to_string(),
            path: path.to_string(),
        }),
    }
}

pub fn read_files(paths: &[&str]) -> Result<Vec<FileDescriptor>, FileReaderError> {
    paths.iter().map(|path| read_file(path)).collect()
}

pub fn write_file(path: &str, contents: &str) -> Result<(), FileWriterError> {
    match fs::write(path, contents) {
        Err(err) => Err(FileWriterError {
            message: err.to_string(),
        }),
        _ => Ok(()),
    }
}

// ==========================
//
// TESTS START
//
// ==========================

#[cfg(test)]
mod tests {
    use crate::fsys::file_reader::{self, FileReaderDescriptor, FileReaderError};

    #[test]
    fn should_return_error_result_if_file_not_found() {
        let result = file_reader::read_file("file/not/found.rs");
        assert_eq!(
            result,
            Err(FileReaderError {
                message: "Failed to read file: No such file or directory (os error 2)".to_string()
            })
        )
    }

    #[test]
    fn should_successfully_read_file() {
        let path = "mock/test.eli";
        let result = file_reader::read_file(path);
        assert_eq!(
            result,
            Ok(FileReaderDescriptor {
                path: path.to_string(),
                content: ".declare(PI 3.14)\n.add(2 + PI)\n".to_string(),
            })
        )
    }
}

// ==========================
//
// TESTS END
//
// ==========================
