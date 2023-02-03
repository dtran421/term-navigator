use std::env;
use std::fs;

use console::style;

fn display_header() {
    println!("[........TERM-NAVIGATOR........]");
    println!("--------------------------------");
}

fn main() -> std::io::Result<()> {
    display_header();

    let path = env::current_dir()?;

    let parent = path.parent();
    let mut parent_str = parent
        .expect("err: parent")
        .to_str()
        .expect("err: parent string")
        .to_owned();
    parent_str.push_str("/");

    let curr = path.file_stem();
    println!(
        "{} The current directory is {}{}",
        style(">").cyan(),
        style(parent_str).blue().dim(),
        style(curr.expect("err: curr").to_str().expect("err: curr string"))
            .magenta()
            .bold()
    );

    let dir = "./";
    let paths = fs::read_dir(dir);

    for entry in paths? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let pathname = path.file_name().unwrap().to_str();
            if !pathname.unwrap().starts_with(".") {
                println!("{}", pathname.unwrap());
            }
        }
    }

    Ok(())
}
