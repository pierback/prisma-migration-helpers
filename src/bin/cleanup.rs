use ansi_term::Colour::{Blue, Yellow};
use ansi_term::Style;
use lib;
use std::fs;
use std::{time::Instant, vec::Vec};

/// Find views in migration directory
/// and remove faulty table creation statements
/// from the latest migration file
fn main() {
    let now = Instant::now();
    println!(
        "{}",
        Blue.bold().paint("Start post migration cleanup 🧹")
    );

    let migration_file_path = lib::get_latest_migration_file_path();
    println!(
        "{} {:?}",
        Style::new().bold().paint(" ->"),
        &migration_file_path
    );

    let migration_dir = lib::get_migration_directory();
    let views = lib::get_views(&migration_dir);

    // get statements from latest migration file as vector of strings
    let mut statement_blocks = lib::get_stmt_blocks(&migration_file_path);

    // filter out statements that target views, e.g. 'create table orders_view'
    let sql_string = filter_sql_string(&mut statement_blocks, views);

    let _ = fs::write(migration_file_path, sql_string);

    println!(
        "\nFinished after: {}",
        Yellow.paint(format!("{:.1?}", now.elapsed()))
    );
}

/// Filters out the create/alter statements from the
/// given stmts vector for the given views names
fn filter_sql_string(stmts: &mut Vec<String>, views: Vec<String>) -> String {
    let mut filtered_stmts: Vec<String> = Vec::new();

    'outer: for stmt in stmts {
        if lib::find_table_stmt(stmt.clone(), views.clone()) {
            continue 'outer;
        }

        filtered_stmts.push(stmt.to_owned());
    }

    // Join and return the filtered statements
    filtered_stmts.join("\n").to_owned().trim().to_string()
}
