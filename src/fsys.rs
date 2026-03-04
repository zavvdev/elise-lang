pub mod file_reader {
    use std::fs;

    #[derive(PartialEq, Debug)]
    pub struct FileReaderDescriptor {
        pub path: String,
        pub content: String,
    }

    #[derive(PartialEq, Debug)]
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

    // ===============================
    // #tests
    // ===============================

    #[cfg(test)]
    mod tests {
        use crate::fsys::file_reader::{self, FileReaderDescriptor, FileReaderError};

        #[test]
        fn should_return_error_result_if_file_not_found() {
            let result = file_reader::read_file("file/not/found.rs");
            assert_eq!(
                result,
                Err(FileReaderError {
                    message: "Failed to read file: No such file or directory (os error 2)"
                        .to_string()
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
}
