use copier::{run, Arguments};

use std::fs::File;
use tempdir::TempDir;

fn fill_directory_for_test(directory: &TempDir, filenames: &mut Vec<String>) {
    let directory_path = directory.path().to_str().unwrap().to_string() + "/";
    for filename in filenames {
        let file_path = directory.path().join(&filename);
        let _ = File::create(file_path);
        filename.insert_str(0, &directory_path);
    }
}

fn get_directory_path(directory: &TempDir) -> String {
    directory.path().to_str().unwrap().to_string() + "/"
}

fn glue_from_to_filenames(from_names: &Vec<String>, to_names: &Vec<String>) -> Vec<String> {
    assert_eq!(from_names.len(), to_names.len());
    let mut result = Vec::new();
    for (from_name, to_name) in from_names.iter().zip(to_names.iter()) {
        result.push(from_name.clone() + " -> " + to_name);
    }
    result.sort();
    result
}

#[test]
fn test_no_pattern() {
    let directory_from = TempDir::new("temporary_test_dir_from").unwrap();
    let directory_to = TempDir::new("temporary_test_dir_to").unwrap();

    let mut filenames = vec![String::from("file1")];
    fill_directory_for_test(&directory_from, &mut filenames);

    let from_string = get_directory_path(&directory_from) + "file1";
    let to_string = get_directory_path(&directory_to) + "file2";

    let args = Arguments {
        pattern_from: from_string.clone(),
        pattern_to: to_string.clone(),
        force: false,
    };

    let result = run(&args);
    assert_eq!(result, Ok(vec![from_string + " -> " + &to_string]));

    let _ = directory_from.close();
    let _ = directory_to.close();
}

#[test]
fn test_with_pattern() {
    let directory_from = TempDir::new("temporary_test_dir_from").unwrap();
    let directory_to = TempDir::new("temporary_test_dir_to").unwrap();

    let mut filenames = vec![
        String::from("file.txt"),
        String::from("a.jpg"),
        String::from("ee.doc"),
        String::from("no_dot"),
    ];
    fill_directory_for_test(&directory_from, &mut filenames);
    filenames.pop();

    let mut new_filenames = vec![
        String::from("new_name_file.txt"),
        String::from("new_name_a.jpg"),
        String::from("new_name_ee.doc"),
    ];
    for filename in &mut new_filenames {
        *filename = get_directory_path(&directory_to) + filename;
    }

    let result_lines = glue_from_to_filenames(&filenames, &new_filenames);

    let from_string = get_directory_path(&directory_from) + "*.*";
    let to_string = get_directory_path(&directory_to) + "new_name_#1.#2";

    let mut args = Arguments {
        pattern_from: from_string.clone(),
        pattern_to: to_string.clone(),
        force: false,
    };

    let result = run(&args);
    assert!(result.is_ok());
    let mut result = result.unwrap();
    result.sort();
    assert_eq!(result, result_lines);

    assert!(run(&args).is_err());

    args.force = true;
    assert!(run(&args).is_ok());

    let _ = directory_from.close();
    let _ = directory_to.close();
}

#[test]
fn test_no_directory_or_path() {
    let mut args = Arguments {
        pattern_from: String::from("does_not_exist_98687qqqr749763r"),
        pattern_to: String::from("a"),
        force: false,
    };
    assert!(run(&args).is_err());

    let directory_from = TempDir::new("temporary_test_dir_from").unwrap();
    args.pattern_from = get_directory_path(&directory_from) + "/*";
    assert!(run(&args).is_err());
}
