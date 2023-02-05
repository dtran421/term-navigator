use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Error};
use std::path::PathBuf;

use clap::Parser;
use dialoguer::{Confirm, Select};
use display::term_obj;

mod display;

#[derive(Parser, Debug)]
#[command(
    author = "Duke Tran",
    version,
    about = "Terminal Navigator",
    long_about = "A simple CLI utility for navigating through your filesystem via the terminal."
)]
struct Args {
    /// List results with numbers
    #[arg(short = 'n', long = "numbered", default_value_t = false)]
    numbered: bool,
}

fn valid_path(entry: &DirEntry) -> bool {
    let path = entry.path();

    if path.is_dir() {
        let pathname = path
            .file_name()
            .expect("file name exists")
            .to_str()
            .expect("file name converts to str");
        if !pathname.starts_with(".") {
            return true;
        }
    }

    return false;
}

fn get_path_options(path: &PathBuf) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(path.to_owned());

    let path_options = paths?
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|entry| valid_path(entry))
        .map(|entry| {
            entry
                .path()
                .file_stem()
                .expect("get file stem from path")
                .to_str()
                .expect("convert path to string")
                .to_owned()
        })
        .collect::<Vec<String>>();
    let default_options = [".".into(), "..".into()].to_vec();

    return Ok([default_options, path_options].concat());
}

fn confirm_path(path: &PathBuf) -> bool {
    let pathname = path.to_str().expect("path string");

    Confirm::with_theme(&term_obj().theme)
        .with_prompt(format!("Confirm navigation to: {}?", &pathname))
        .interact_on(&term_obj().term)
        .expect("confirm prompt displays and interactable")
}

fn handle_selection(
    origin: &PathBuf,
    path: &mut PathBuf,
    options: &Vec<String>,
    index: usize,
) -> bool {
    match index {
        0 => {
            if path.to_str().eq(&origin.to_str()) {
                return false;
            }

            let confirm = confirm_path(&path);
            if confirm {
                term_obj().term.clear_screen().expect("console cleared");
                println!("{}", path.to_str().expect("path string"));
                return false;
            }
        }
        1 => {
            path.pop();
        }
        idx => {
            path.push(options[idx].to_string());
        }
    };

    return true;
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    println!("{}", args.numbered);

    let origin = env::current_dir()?;
    let mut path = env::current_dir()?;

    let mut repl = true;
    while repl {
        term_obj().term.clear_screen()?;
        display::display_header(&origin, &path);

        let options = get_path_options(&path)?;

        let selection = Select::with_theme(&term_obj().theme)
            .items(&options)
            .default(0)
            .interact_on_opt(&term_obj().term)?;

        repl = match selection {
            Some(index) => handle_selection(&origin, &mut path, &options, index),
            None => {
                term_obj().term.write_line("User did not select anything")?;
                false
            }
        };
    }

    Ok(())
}
