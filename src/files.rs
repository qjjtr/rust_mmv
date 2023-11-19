use std::collections::HashMap;
use std::fs;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use tempdir::TempDir;

pub fn get_filenames_in_directory(directory_name: &String) -> Result<Vec<String>, String> {
    let mut filenames = Vec::new();
    let directory = fs::read_dir(directory_name);
    let directory = match directory {
        Ok(dir) => dir,
        Err(_e) => return Err(String::from("can't open directory")),
    };
    for file in directory {
        let file = file.unwrap().path().to_str().unwrap().to_string();
        filenames.push(file);
    }
    Ok(filenames)
}

pub fn copy_files(converted_names: &HashMap<String, String>) {
    for (initial_name, new_name) in converted_names {
        let _ = fs::copy(initial_name, new_name);
    }
}

#[cfg(test)]
pub fn fill_directory_for_test(directory: &TempDir) -> Vec<String> {
    let directory_path = directory.path().to_str().unwrap().to_string() + "/";
    let mut filenames = vec![
        String::from("aboba"),
        String::from("cat"),
        String::from("file"),
        String::from("yellow"),
    ];
    for filename in &mut filenames {
        let file_path = directory.path().join(&filename);
        let _ = File::create(file_path);
        filename.insert_str(0, &directory_path);
    }
    filenames
}

#[test]
fn test_get_filenames_in_directory_good() {
    let directory = TempDir::new("temporary_test_dir").unwrap();
    let directory_path = directory.path().to_str().unwrap().to_string() + "/";

    assert_eq!(
        get_filenames_in_directory(&directory_path),
        Ok(Vec::<String>::new())
    );

    let filenames = fill_directory_for_test(&directory);

    let mut result = get_filenames_in_directory(&directory_path).unwrap();
    result.sort();
    assert_eq!(result, filenames);

    let _ = directory.close();
}

#[test]
fn test_get_filenames_in_directory_panic() {
    let directory = TempDir::new("temporary_test_dir").unwrap();
    let directory_path = directory.path().to_str().unwrap().to_string() + "/i_am_not_exist";

    assert!(get_filenames_in_directory(&directory_path).is_err());
}

#[test]
fn test_copy_files() {
    let directory = TempDir::new("temporary_test_dir").unwrap();
    let directory_path = directory.path().to_str().unwrap().to_string() + "/";

    let new_directory = TempDir::new("temporary_new_home").unwrap();
    let new_directory_path = new_directory.path().to_str().unwrap().to_string() + "/";

    let mut filenames = fill_directory_for_test(&directory);
    let mut converted_names = HashMap::new();
    for filename in &mut filenames {
        let old_name = filename.clone();
        *filename = filename.replace(&directory_path, &new_directory_path);
        converted_names.insert(old_name, filename.clone());
    }

    copy_files(&converted_names);

    filenames.sort();
    let mut new_filenames = get_filenames_in_directory(&new_directory_path).unwrap();
    new_filenames.sort();

    assert_eq!(filenames, new_filenames);
}
