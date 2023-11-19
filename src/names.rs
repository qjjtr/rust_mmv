use crate::files::get_filenames_in_directory;
use crate::pattern::is_matched_by_pattern;

#[cfg(test)]
use crate::files::fill_directory_for_test;
#[cfg(test)]
use tempdir::TempDir;

fn get_directory(full_name: &String) -> String {
    let delimeter = full_name.rfind('/');
    if let Some(delimeter) = delimeter {
        full_name.get(0..delimeter + 1).unwrap().to_string()
    } else {
        String::from("./")
    }
}

pub fn get_names_by_pattern(pattern: &String) -> Result<Vec<String>, String> {
    let directory = get_directory(pattern);
    let mut matched_names = Vec::new();

    let files_in_directory = get_filenames_in_directory(&directory);
    let files_in_directory = match files_in_directory {
        Ok(files) => files,
        Err(error) => return Err(error),
    };

    for file in files_in_directory {
        if is_matched_by_pattern(&file, &pattern) {
            matched_names.push(file);
        }
    }
    Ok(matched_names)
}

pub fn has_any_file(names: &Vec<String>) -> Result<bool, String> {
    if names.is_empty() {
        return Ok(false);
    }
    let directory = get_directory(&names[0]);

    let files_in_directory = get_filenames_in_directory(&directory);
    let files_in_directory = match files_in_directory {
        Ok(files) => files,
        Err(error) => return Err(error),
    };

    for name in names {
        if files_in_directory.contains(name) {
            return Ok(true);
        }
    }
    Ok(false)
}

#[test]
fn test_get_directory() {
    assert_eq!(
        get_directory(&String::from("/root/path/dir/file")),
        String::from("/root/path/dir/")
    );
    assert_eq!(get_directory(&String::from("/file")), String::from("/"));
    assert_eq!(get_directory(&String::from("file")), String::from("./"));
    assert_eq!(get_directory(&String::from("/dir/")), String::from("/dir/"));
}

#[test]
fn test_get_names_by_pattern() {
    let directory = TempDir::new("temporary_test_dir").unwrap();
    let directory_path = directory.path().to_str().unwrap().to_string() + "/";

    let mut filenames = fill_directory_for_test(&directory);
    filenames.truncate(2);
    assert_eq!(
        get_names_by_pattern(&(directory_path + &String::from("*a*"))).unwrap(),
        filenames
    );

    let _ = directory.close();
}

#[test]
fn test_has_any_file() {
    assert!(!has_any_file(&Vec::new()).unwrap());

    let directory = TempDir::new("temporary_test_dir").unwrap();
    let directory_path = directory.path().to_str().unwrap().to_string() + "/";

    let filenames = fill_directory_for_test(&directory);

    assert!(has_any_file(&filenames).unwrap());
    assert!(!has_any_file(&vec![directory_path + "nonononono"]).unwrap());

    let _ = directory.close();
}
