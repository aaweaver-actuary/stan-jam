use crate::stan_model::StanModel;
use std::fs::read_to_string;
use std::io::Error;
use tempfile::tempdir;

pub type FolderList = Vec<String>;
pub type FileList = Vec<String>;

pub trait SourceParser {
    fn get_folders(&self) -> Option<FolderList>;
    fn get_files_in_folder(&self, folder_index: usize) -> Option<FileList>;
    fn get_files_in_folders(&self) -> Option<FileList>;
    fn find_file_in_folders(&self) -> Option<String>;
    fn read_file(&self) -> Result<StanModel, Error>;
}

#[derive(Debug, PartialEq)]
pub struct StanSourceParser {
    pub filename: String,
    pub folders: FolderList,
}

impl StanSourceParser {
    pub fn new(filename: &str) -> StanSourceParser {
        StanSourceParser {
            filename: filename.to_string(),
            folders: vec![".".to_string()],
        }
    }

    pub fn add_folder(&mut self, folder: &str) {
        self.folders.push(folder.to_string());
    }

    pub fn read_file_contents(&self) -> Result<String, Error> {
        read_to_string(&self.filename)
    }

    pub fn get_lines(&self) -> Vec<String> {
        let file_contents = self.read_file_contents().unwrap();
        file_contents
            .lines()
            .map(|s| s.to_string().trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /*     pub fn parse_includes(&self) -> Vec<String> {
        let mut includes = Vec::new();
        let model = StanModel::new();
        if let Some(functions) = &model.functions {
            for line in functions {
                if line.starts_with("#include") {
                    let include = line.split_whitespace().last().unwrap();
                    includes.push(include.to_string());
                }
            }
        }
        includes
    } */
}

#[cfg(test)]
mod tests {
    use std::fs::write;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use crate::stan_source_parser::dir_for_tests::create_temp_directory_structure;
    use crate::{stan_model_block::StanModelBlock, stan_model_block_type::StanModelBlockType};

    use super::*;

    /// TestSourceParser is a testing-only struct that implements the SourceParser trait.
    struct TestSourceParser {
        pub _filename: String,
        pub folders: FolderList,
    }

    /*     impl TestSourceParser {
        fn parse_includes(&self) -> Vec<String> {
            let mut includes = Vec::new();
            let model = self.read_file().unwrap();
            if let Some(functions) = &model.functions {
                for line in functions {
                    if line.starts_with("#include") {
                        let include = line.split_whitespace().last().unwrap();
                        includes.push(include.to_string());
                    }
                }
            }
            includes
        }
    } */

    impl SourceParser for TestSourceParser {
        fn get_folders(&self) -> Option<FolderList> {
            Some(vec![
                "folder1".to_string(),
                "folder2".to_string(),
                "folder1/subfolder".to_string(),
            ])
        }

        fn get_files_in_folder(&self, _folder_index: usize) -> Option<FileList> {
            Some(vec!["test.stan".to_string(), "functions.stan".to_string()])
        }

        fn get_files_in_folders(&self) -> Option<FileList> {
            Some(vec!["test.stan".to_string(), "functions.stan".to_string()])
        }

        fn find_file_in_folders(&self) -> Option<String> {
            Some("test.stan".to_string())
        }

        fn read_file(&self) -> Result<StanModel, Error> {
            let mut model = StanModel::new();
            model.add_function("#include <functions.stan>");
            model.add_data("int<lower=0> N;");
            model.add_parameter("real mu;");
            model.add_model("mu ~ normal(0, 1);");

            Ok(model)
        }
    }

    #[test]
    fn test_all_methods_in_the_test_source_parser() {
        let parser = TestSourceParser {
            _filename: "test.stan".to_string(),
            folders: vec![".".to_string()],
        };

        assert_eq!(
            parser.get_folders(),
            Some(vec![
                "folder1".to_string(),
                "folder2".to_string(),
                "folder1/subfolder".to_string(),
            ])
        );

        let folder_contents = vec!["test.stan".to_string(), "functions.stan".to_string()];

        assert_eq!(parser.get_files_in_folder(0), Some(folder_contents.clone()));
        assert_eq!(parser.get_files_in_folders(), Some(folder_contents.clone()));
        assert_eq!(parser.find_file_in_folders(), Some("test.stan".to_string()));

        let model = parser.read_file().unwrap();

        let mut functions_block = StanModelBlock::new(StanModelBlockType::Functions);
        functions_block.add("#include <functions.stan>");

        let mut data_block = StanModelBlock::new(StanModelBlockType::Data);
        data_block.add("int<lower=0> N;");

        let mut parameter_block = StanModelBlock::new(StanModelBlockType::Parameters);
        parameter_block.add("real mu;");

        let mut model_block = StanModelBlock::new(StanModelBlockType::Model);
        model_block.add("mu ~ normal(0, 1);");

        assert_eq!(model.functions, Some(functions_block));
        assert_eq!(model.data, data_block);
        assert_eq!(model.parameters, parameter_block);
        assert_eq!(model.model, model_block);
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
    fn can_add_folders_to_parser_search_list() {
        let mut parser = StanSourceParser::new("test.stan");
        parser.add_folder("folder1");
        parser.add_folder("folder2");

        assert_eq!(
            parser.folders,
            vec![
                ".".to_string(),
                "folder1".to_string(),
                "folder2".to_string()
            ]
        );
    }

    #[test]
    fn can_find_files_in_folders() {
        let parser = TestSourceParser {
            _filename: "test.stan".to_string(),
            folders: vec![".".to_string()],
        };
        assert_eq!(parser.folders, vec![".".to_string()]);
    }

    #[test]
    fn can_read_a_file_contents() {
        let temp_dir = create_temp_directory_structure();
        let test_file = temp_dir.path().join("test_model.stan");

        let test_file_contents = r#"
            functions {
                #include "functions/functions.stan"
                #include "helpers.stan"
            }
            data {
                #include "data/data.stan"
            }
            model {
                y ~ normal(0, 1);
            }
            "#;

        write(&test_file, test_file_contents).unwrap();

        let parser = StanSourceParser::new(test_file.to_str().unwrap());
        let file_contents = parser.read_file_contents().unwrap();

        assert_eq!(file_contents, test_file_contents);
    }

    fn setup_test_dir() -> (TempDir, PathBuf, String) {
        let temp_dir = create_temp_directory_structure();
        let test_file = temp_dir.path().join("test_model.stan");

        let test_file_contents = r#"
            functions {
                #include "functions/functions.stan"
                #include "helpers.stan"
            }
            data {
                #include "data/data.stan"
            }
            model {
                y ~ normal(0, 1);
            }
            "#;

       (temp_dir, test_file, test_file_contents.to_owned())
    }

    #[test]
    fn can_split_a_file_into_a_vec_of_lines() {
        let (temp_dir, test_file, test_file_contents) = setup_test_dir();

        write(&test_file, test_file_contents).unwrap();

        let parser = StanSourceParser::new(test_file.to_str().unwrap());
        let lines = parser.get_lines();

        assert_eq!(lines[0], "functions {");
        assert_eq!(lines[1], "#include \"functions/functions.stan\"");
        assert_eq!(lines[2], "#include \"helpers.stan\"");
        assert_eq!(lines[3], "}");
        assert_eq!(lines[4], "data {");
        assert_eq!(lines[5], "#include \"data/data.stan\"");
        assert_eq!(lines[6], "}");
        assert_eq!(lines[7], "model {");
        assert_eq!(lines[8], "y ~ normal(0, 1);");
        assert_eq!(lines[9], "}");
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn can_find_lines_with_an_include_directive() {
        let parse
    }

    /*     #[test]
    fn can_detect_first_include_directive_in_a_block() {
        let temp_dir = create_temp_directory_structure();

        let mut functions_parser = StanSourceParser::new("test_model.stan");
        functions_parser.add_folder("functions");

        let model = functions_parser.read_file_contents().unwrap();

    } */

    /*          #[test]
    fn can_detect_and_capture_all_include_directives() {
        // confirm that the parser can detect and capture all #include directives
        // using parser.parse_includes() method
        let temp_dir = create_temp_directory_structure();

        let mut functions_file_parser = StanSourceParser::new("functions.stan");
        functions_file_parser.add_folder("functions");

        let mut data_file_parser = StanSourceParser::new("data.stan");
        data_file_parser.add_folder("data");

        let mut helpers_file_parser = StanSourceParser::new("helpers.stan");

    }  */
}
