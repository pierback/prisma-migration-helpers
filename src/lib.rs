use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::{path::Path, path::PathBuf, vec::Vec};
use walkdir::WalkDir;

/// Reads contents of a file and splits them by `;` returning a vector of statements
pub fn get_stmt_blocks(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines = contents
        .split_inclusive(";")
        .into_iter()
        .map(|v| v.to_owned())
        .collect::<Vec<String>>();

    return lines;
}

/// Based on given statement string returns true,
/// if the statement is of type `alter` or `create`
/// and targets at least one of the given views
pub fn find_table_stmt(stmt: String, views: Vec<String>) -> bool {
    // vector of "words", e.g. [..., "create", "table", "bvdh_ledger_entries", ...]
    let code_block = split_stmt_string(stmt.clone());

    for view in views.iter() {
        // [..., "create", "table", "orders_view", ...]
        let create_table_stmt_vec = operation_stmt_template(&"create", &view);
        if includes_sub_vec(&code_block, &create_table_stmt_vec) {
            return true;
        }
        // [..., "alter", "table", "orders_view", ...]
        let alter_table_stmt_vec = operation_stmt_template(&"alter", &view);
        if includes_sub_vec(&code_block, &alter_table_stmt_vec) {
            return true;
        }
    }

    return false;
}

/// Splits a statement string by `\n` and ` `
/// Returns a vector of strings, i.e. tokens
pub fn split_stmt_string(stmt: String) -> Vec<String> {
    stmt.split("\n")
        .into_iter()
        // split line string by whitespace
        .flat_map(|v| v.split(" ").to_owned())
        .map(|v| v.to_owned().to_lowercase())
        .collect()
}

/// Returns a template as vector for a given operation and view
///
/// # Examples
///
/// ```
/// CREATE TABLE "orders_view"
/// ```
pub fn operation_stmt_template(operation: &str, view_name: &str) -> Vec<String> {
    vec![operation, "table", &format!("\"{}\"", view_name)]
        .into_iter()
        .map(|v| v.to_owned())
        .collect::<Vec<String>>()
}

/// Checks if the given vector contains the given sub vector
pub fn includes_sub_vec<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 {
        return true;
    }
    
    while !haystack.is_empty() {
        if haystack.starts_with(needle) {
            return true;
        }
        haystack = &haystack[1..];
    }
    
    return false;
}

/// Returns migration directory based on the current working directory
pub fn get_migration_directory() -> String {
    let path = std::env::current_dir().unwrap();

    for dir_entry_result in WalkDir::new(path).into_iter() {
        if let Some(dir) = is_migration_dir(dir_entry_result) {
            return dir;
        }
    }

    println!("Migrations dir not found!");
    std::process::exit(1);
}

/// Returns the path of the latest migration file
pub fn get_latest_migration_file_path() -> &'static str {
    let mig_dir = get_migration_directory();

    let mut mig_dirs = fs::read_dir(mig_dir)
        .unwrap()
        .map(|r| r.map(|d| d.path()))
        .filter(|r| r.is_ok() && r.as_deref().unwrap().is_dir())
        .map(|r| r.unwrap())
        .collect::<Vec<PathBuf>>();

    // sort the directories by name asc
    mig_dirs.sort();

    // pick latest migration directory from vector
    let latest = mig_dirs.last().unwrap().to_owned();
    let path = Path::new(&latest).join("migration.sql");

    // store path string on heap
    let boxed_path_string = Box::new(path.to_str().unwrap().to_owned());

    // create path string with static lifetime
    Box::leak(boxed_path_string)
}

/// Walks through the given migration directory and returns a vector of migration files
pub fn get_views(migration_dir: &str) -> Vec<String> {
    let mut views: HashMap<String, bool> = HashMap::new();

    WalkDir::new(migration_dir)
        .into_iter()
        .for_each(|dir_entry_result| {
            if let Some(path) = is_sql_file(dir_entry_result) {
                // read file and lowercase output string
                let sql_string = std::fs::read_to_string(path).unwrap().to_lowercase();
                if sql_string.contains("materialized view") || sql_string.contains("view") {
                    let view_name = extract_view_name(&sql_string);
                    views.insert(view_name, true);
                }
            }
        });

    return views.into_keys().collect::<Vec<String>>();
}

/// Returns path if dir_entry corresponds to a sql file
pub fn is_sql_file(dir_entry_result: Result<walkdir::DirEntry, walkdir::Error>) -> Option<String> {
    let dir_entry = dir_entry_result.unwrap();

    // check if entry is a file
    if dir_entry.file_type().is_file() {
        let path = dir_entry.path();

        // check if file is a sql file
        if let Some(extension) = path.extension() {
            if extension == "sql" {
                return Some(path.to_owned().to_str().unwrap().to_owned());
            }
        }
    }

    return None;
}

/// Returns path if dir_entry corresponds to a sql file
pub fn is_migration_dir(
    dir_entry_result: Result<walkdir::DirEntry, walkdir::Error>,
) -> Option<String> {
    let dir_entry = dir_entry_result.unwrap();

    // check if entry is a file
    if dir_entry.file_type().is_dir() {
        let path = dir_entry.path();

        // check if folder name is migrations
        if let Some(folder_name) = path.file_name() {
            if folder_name == "migrations" {
                return Some(path.to_owned().to_str().unwrap().to_owned());
            }
        }
    }

    return None;
}

/// Extracts view name from the given sql string
pub fn extract_view_name(sql_string: &str) -> String {
    Regex::new(r"view ([^\s]+) as")
        .unwrap()
        .captures(&sql_string)
        .unwrap()
        .get(0)
        .map_or("", |m| m.as_str())
        .replace("view ", "")
        .replace(" as", "")
}
