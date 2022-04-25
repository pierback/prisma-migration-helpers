use lib;
use std::{path::PathBuf, vec::Vec};

/// Get next version number for newest migration file
fn main() {
    let latest_migration_file_path = PathBuf::from(lib::get_latest_migration_file_path());

    let latest_migration_folder_path = latest_migration_file_path
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    // split latest_migration_folder_path into vector of strings
    let path_vec = latest_migration_folder_path
        .split("/")
        .collect::<Vec<&str>>();

    // get the last element of the vector -> name of migration folder
    let latest_mig_folder_name = path_vec.last().unwrap().to_owned();
    let name_vec = latest_mig_folder_name.split("_").collect::<Vec<&str>>();

    let current_version = name_vec.last().unwrap().to_owned();

    // convert version string to integer and add 1
    let new_version = current_version.parse::<i32>().unwrap() + 1;

    println!("{}", new_version);
}
