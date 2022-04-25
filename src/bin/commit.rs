use lib;
use std::{env, path::Path, vec::Vec};

fn main() {
    let output = std::process::Command::new("git")
        .arg("diff")
        .arg("--staged")
        .arg("--name-only")
        .output()
        .unwrap();

    // Get the list of files that have been staged
    let commit_list_string = String::from_utf8_lossy(&output.stdout);

    // split string into vector of strings and filter out 'migration.sql' files
    let sql_file_list = commit_list_string
        .split("\n")
        .into_iter()
        .map(|v| v.to_owned())
        .filter(|v| v.contains("migration.sql"))
        .collect::<Vec<String>>();

    if sql_file_list.len() > 0 {
        // get absolute path of migration file
        let cwd = env::current_dir().unwrap();
        let mig_file = Path::new(&cwd)
            .join(&sql_file_list[0])
            .to_str()
            .unwrap()
            .to_owned();

        let stmts = lib::get_stmt_blocks(&mig_file);
        let migration_dir = lib::get_migration_directory();
        let views = lib::get_views(&migration_dir);

        // traverse the stmt blocks and check if any of them are a create table statement
        for stmt in stmts {
            if lib::find_table_stmt(stmt.clone(), views.clone()) {
                println!(
                    "Statement found for existing view in {}\n Please remove!",
                    &mig_file
                );
                std::process::exit(1);
            }
        }
    }
}
