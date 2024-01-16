use std::fs::File;
use std::io::Write;
extern crate chrono;

fn main() {
    let cargo_toml_content = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let version = cargo_toml_content
        .lines()
        .find(|line| line.starts_with("version = "))
        .and_then(|line| {
            Some(line.trim_start_matches("version = ")
                .trim_matches('"')
                .trim_matches('"')
                .to_string())
        }).unwrap();

    let now_local = chrono::Local::now();
    
    let mut file = File::create("include.rs").expect("Unable to create file");
    writeln!(file, "pub const VERSION: &str = \"{}\";", version).expect("Failed to write version file");
    writeln!(file, "pub const BUILD_DATE: &str = \"{}\";", now_local.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap();
}
