use std::fs::{create_dir_all, write};
use tempfile::tempdir;

pub fn create_temp_directory_structure() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();
    let top_level_file = temp_dir.path().join("test_model.stan");

    let functions_dir = temp_dir.path().join("functions");
    let functions_file = functions_dir.as_path().join("functions.stan");

    let data_dir = temp_dir.path().join("data");
    let data_file = data_dir.as_path().join("data.stan");

    let helpers_file = temp_dir.path().join("helpers.stan");

    // Create the directories and files
    create_dir_all(functions_dir).unwrap();
    create_dir_all(data_dir).unwrap();

    let top_level_file_contents = r#"
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

    write(top_level_file, top_level_file_contents).unwrap();

    write(
        functions_file,
        "real functions_file_function(real x) {\n  return x;\n}",
    )
    .unwrap();
    write(data_file, "int<lower=0> data_file_datum;").unwrap();
    write(
        helpers_file,
        "real helpers_file_function(real x) {\n  return x;\n}",
    )
    .unwrap();

    temp_dir
}
