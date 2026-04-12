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

    use crate::fsys::{
        FileDescriptor, FileReaderError, FileWriterError, read_file, read_files, write_file,
    };
    // We need to use this crate here in order to make these tests run in serial order.
    // If we run them in parallel, we might end up in a situation when our tests
    // that expect some file to not be created has already been created by another test.
    use serial_test::serial;
    use std::fs;
    use std::path::Path;

    #[test]
    #[serial]
    fn read_file_reads_successfully() {
        let file_name = "test.eli";
        let contents = ".declare(PI 3.14)\n.add(2 + PI)\n";

        fs::write(file_name, contents).expect("Cannot create test file");
        let result = read_file(file_name);

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
    fn read_file_returns_error_if_not_found() {
        let file_name = "test.eli";
        let result = read_file(file_name);
        assert_eq!(
            result,
            Err(FileReaderError {
                message: "No such file or directory (os error 2)".to_string(),
                path: file_name.to_string(),
            })
        );
    }

    #[test]
    #[serial]
    fn read_files_reads_multiple_files_successfully() {
        let file_name1 = "test.eli";
        let file_name2 = "test2.eli";

        let contents1 = ".declare(PI 3.14)\n.add(2 + PI)\n";
        let contents2 = ".declare(PI 3.14)\n.add(2 + PI)\n";

        fs::write(file_name1, contents1).expect("Cannot write test file #1");
        fs::write(file_name2, contents2).expect("Cannot write test file #2");

        let result = read_files(&[&file_name1, &file_name2]);

        assert_eq!(
            result,
            Ok(vec![
                FileDescriptor {
                    path: file_name1.to_string(),
                    content: contents1.to_string(),
                },
                FileDescriptor {
                    path: file_name2.to_string(),
                    content: contents2.to_string(),
                }
            ])
        );

        fs::remove_file(file_name1).expect("Failed to delete test file #1");
        fs::remove_file(file_name2).expect("Failed to delete test file #2");
    }

    #[test]
    #[serial]
    fn read_files_returns_error_if_any_not_found() {
        let file_name1 = "test.eli";
        let file_name2 = "test2.eli";
        let contents1 = ".declare(PI 3.14)\n.add(2 + PI)\n";

        fs::write(file_name1, contents1).expect("Cannot write test file #1");

        let result = read_files(&[&file_name1, &file_name2]);

        assert_eq!(
            result,
            Err(FileReaderError {
                message: "No such file or directory (os error 2)".to_string(),
                path: file_name2.to_string(),
            })
        );

        fs::remove_file(file_name1).expect("Failed to delete test file #1");
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

    #[test]
    #[serial]
    fn write_file_errors_when_cannot_write() {
        let file_name = "test.eli";
        let contents = ".declare(PI 3.14)\n.add(2 + PI)\n";
        fs::write(file_name, "").expect("Cannot write test file #1");

        let mut perms = fs::metadata(file_name).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(file_name, perms).unwrap();

        let result = write_file(file_name, contents);

        assert_eq!(
            result,
            Err(FileWriterError {
                message: "Permission denied (os error 13)".to_string()
            })
        );

        let mut perms = std::fs::metadata(file_name).unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(file_name, perms).unwrap();
        fs::remove_file(file_name).expect("Failed to delete test file");
    }
}

// ==========================
//
// TESTS END
//
// ==========================
