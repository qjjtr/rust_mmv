mod files;
mod names;
mod pattern;

use files::copy_files;
use names::{get_names_by_pattern, has_any_file};
use pattern::convert;

use clap::Parser;

/// Arguments to program
///
/// Accept either absolute or relative path to files
#[derive(Parser)]
pub struct Arguments {
    /// string argument to match files that needed to copy
    ///
    /// use * to capture any string
    ///
    /// don't use * in directory names
    pub pattern_from: String,

    /// string argument that describes how to rename files
    ///
    /// use #\<number\> to put captured strings from pattern_from in order of capturing
    ///
    /// target directory should exist
    pub pattern_to: String,

    #[arg(short, long, default_value_t = false)]
    /// bool argument, if it is set to true, program will replace existing files if there are some
    /// Otherwise no files will be moved and error message will be shown
    pub force: bool,
}

/// Main function, releases program logic
///
/// Copies files with rules written in documentation for structure `Arguments`
///
/// If there are some errors during execution, string with error message is returned
/// Otherwise `Vec<String>` with copy log is returned
///
/// Every line in log looks like
/// `<old_filename> -> <new_filename>`
pub fn run(arguments: &Arguments) -> Result<Vec<String>, String> {
    let matched_names = get_names_by_pattern(&arguments.pattern_from);
    let matched_names = match matched_names {
        Ok(names) => names,
        Err(error) => return Err(error),
    };

    if matched_names.is_empty() {
        return Err(String::from("found no files matching pattern"));
    }
    let converted_names = convert(
        matched_names,
        &arguments.pattern_from,
        &arguments.pattern_to,
    );

    let has_files = has_any_file(&converted_names.clone().into_values().collect());
    let has_files = match has_files {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    if !arguments.force && has_files {
        return Err(String::from(
            "files with target names are already exist. run programm with --froce flag",
        ));
    }

    copy_files(&converted_names);
    let mut result_output = Vec::new();
    for (old_name, new_name) in converted_names {
        result_output.push(old_name + " -> " + &new_name);
    }
    Ok(result_output)
}
