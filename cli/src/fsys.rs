//! # File system module
//!
//! This module is an abstraction around file system
//! which exposes functions that can be useful for
//! preparing program for running or handling
//! program output.

use std::fs;

#[derive(PartialEq, Debug)]
pub struct FileDescriptor<T> {
    pub path: String,
    pub content: T,
}

#[derive(PartialEq, Debug)]
pub struct FileRwErr {
    pub message: String,
    pub path: String,
}

pub fn read_file_string(path: &str) -> Result<FileDescriptor<String>, FileRwErr> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(FileDescriptor {
            path: path.to_string(),
            content,
        }),
        Err(e) => Err(FileRwErr {
            message: e.to_string(),
            path: path.to_string(),
        }),
    }
}

pub fn read_file_bytes(path: &str) -> Result<FileDescriptor<Vec<u8>>, FileRwErr> {
    match fs::read(path) {
        Ok(content) => Ok(FileDescriptor {
            path: path.to_string(),
            content,
        }),
        Err(e) => Err(FileRwErr {
            message: e.to_string(),
            path: path.to_string(),
        }),
    }
}

pub fn write_file(path: &str, contents: &str) -> Result<(), FileRwErr> {
    match fs::write(path, contents) {
        Err(err) => Err(FileRwErr {
            message: err.to_string(),
            path: path.to_string(),
        }),
        _ => Ok(()),
    }
}

// ==================================================================
//
// TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {

    use crate::fsys::{FileDescriptor, FileRwErr, read_file_bytes, read_file_string, write_file};
    // We need to use this crate here in order to make these tests run in serial order.
    // If we run them in parallel, we might end up in a situation when our tests
    // that expect some file to not be created has already been created by another test.
    use serial_test::serial;
    use std::fs;
    use std::path::Path;

    #[test]
    #[serial]
    fn read_file_string_should_read_successfully() {
        let file_name = "test.eli";
        let contents = ".declare(PI 3.14)\n.add(2 + PI)\n";

        fs::write(file_name, contents).expect("Cannot create test file");
        let result = read_file_string(file_name);

        assert_eq!(
            result,
            Ok(FileDescriptor {
                path: file_name.to_string(),
                content: contents.to_string(),
            })
        );
        fs::remove_file(file_name).expect("Failed to delete test file");
    }

    #[test]
    #[serial]
    fn read_file_string_should_return_error_if_not_found() {
        let file_name = "test.eli";
        let result = read_file_string(file_name);
        assert_eq!(
            result,
            Err(FileRwErr {
                message: "No such file or directory (os error 2)".to_string(),
                path: file_name.to_string(),
            })
        );
    }

    #[test]
    #[serial]
    fn read_file_bytes_should_read_successfully() {
        let file_name = "test.eli";
        let contents = "abc";

        fs::write(file_name, contents).expect("Cannot create test file");
        let result = read_file_bytes(file_name);

        assert_eq!(
            result,
            Ok(FileDescriptor {
                path: file_name.to_string(),
                content: vec![97, 98, 99],
            })
        );
        fs::remove_file(file_name).expect("Failed to delete test file");
    }

    #[test]
    #[serial]
    fn read_file_bytes_should_return_error_if_not_found() {
        let file_name = "test.eli";
        let result = read_file_string(file_name);
        assert_eq!(
            result,
            Err(FileRwErr {
                message: "No such file or directory (os error 2)".to_string(),
                path: file_name.to_string(),
            })
        );
    }

    #[test]
    #[serial]
    fn write_file_writes_to_new_file() {
        let file_name = "test.eli";
        let contents = ".declare(PI 3.14)\n.add(2 + PI)\n";
        let result = write_file(file_name, contents);
        assert_eq!(result, Ok(()));
        assert!(Path::new(file_name).exists());
        fs::remove_file(file_name).expect("Failed to delete test file");
    }

    #[test]
    #[serial]
    fn write_file_writes_to_existing_file() {
        let file_name = "test.eli";
        let contents = ".declare(PI 3.14)\n.add(2 + PI)\n";
        fs::write(file_name, "").expect("Cannot write test file #1");
        let result = write_file(file_name, contents);
        assert_eq!(result, Ok(()));
        fs::remove_file(file_name).expect("Failed to delete test file");
    }
}

// ==================================================================
//
// TESTS END
//
// ==================================================================
