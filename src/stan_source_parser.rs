use std::io::Error;

pub trait SourceParser {
    fn get_folders(&self) -> Option<Vec<String>>;
    fn get_files_in_folder(&self, folder_index: usize) -> Option<Vec<String>>;
    fn get_files_in_folders(&self) -> Option<Vec<String>>;
    fn find_file_in_folders(&self) -> Option<String>;
    fn read_file(&self) -> Result<Vec<String>, Error>;
}

#[derive(Debug, PartialEq)]
pub struct StanSourceParser {
    pub filename: String,
    pub folders: Vec<String>,
}

impl StanSourceParser {
    pub fn new(filename: &str) -> StanSourceParser {
        StanSourceParser {
            filename: filename.to_string(),
            folders: vec![".".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSourceParser {
        pub folders: Vec<String>,
    }

    impl SourceParser for TestSourceParser {
        fn get_folders(&self) -> Option<Vec<String>> {
            Some(vec![
                "folder1".to_string(),
                "folder2".to_string(),
                "folder1/subfolder".to_string(),
            ])
        }

        fn get_files_in_folder(&self, folder_index: usize) -> Option<Vec<String>> {
            Some(vec!["test.stan".to_string()])
        }

        fn get_files_in_folders(&self) -> Vec<String> {
            vec!["test.stan".to_string()]
        }

        fn find_file_in_folders(&self) -> Option<String> {
            Some("test.stan".to_string())
        }

        fn read_file(&self) -> Result<Vec<String>, Error> {
            Ok(vec!["test.stan".to_string()])
        }
    }

    #[test]
    fn test_stan_source_parser() {
        let parser1 = StanSourceParser::new("test.stan");
        assert_eq!(parser1.filename, "test.stan");

        let parser2 = StanSourceParser {
            filename: "test.stan".to_string(),
            folders: vec![".".to_string()],
        };

        assert_eq!(parser1, parser2);
    }

    #[test]
    fn can_find_files_in_folders() {
        let parser = TestSourceParser {
            filename: "test.stan".to_string(),
            folders: vec![".".to_string()],
        };
        assert_eq!(parser.folders, vec![".".to_string()]);
    }
}
