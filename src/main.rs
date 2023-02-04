use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Error};
use std::path::PathBuf;

use clap::Parser;
use console::{self, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

struct TermObj {
    term: Term,
    theme: ColorfulTheme,
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

fn confirm_path(term_obj: &TermObj, path: &PathBuf) -> bool {
    let pathname = path.to_str().expect("path string");

    Confirm::with_theme(&term_obj.theme)
        .with_prompt(format!("Confirm navigation to: {}?", pathname))
        .interact_on(&term_obj.term)
        .expect("confirm prompt displays and interactable")
}

fn handle_selection(
    term_obj: &TermObj,
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

            let confirm = confirm_path(&term_obj, &path);
            if confirm {
                term_obj.term.clear_screen().expect("console cleared");
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

fn display_curr_dir(term: &Term, path: &PathBuf) {
    let parent = path.parent().expect("path parent exists");
    let mut parent_str = parent.to_str().expect("parent string exists").to_owned();
    parent_str.push_str("/");

    let curr = path
        .file_stem()
        .expect("path file stem exists")
        .to_str()
        .expect("file stem to str");

    let formatted_path_str = format!(
        "{} The current directory is {}{}",
        console::style(">").cyan(),
        console::style(parent_str).blue().dim(),
        console::style(curr).magenta().bold()
    );
    term.write_line(formatted_path_str.as_str())
        .expect("console write");
}

fn display_header(term: &Term, path: &PathBuf) {
    term.write_line("[........TERM-NAVIGATOR........]")
        .expect("console write");
    display_curr_dir(&term, &path);
    term.write_line("--------------------------------")
        .expect("console write");
}

fn main() -> io::Result<()> {
    let term_obj = TermObj {
        term: Term::stderr(),
        theme: ColorfulTheme::default(),
    };

    console::set_colors_enabled_stderr(true);

    let origin = env::current_dir()?/* PathBuf::from(&args[1]) */;
    let mut path = env::current_dir()?/* PathBuf::from(&args[1]) */;

    let mut repl = true;
    while repl {
        term_obj.term.clear_screen()?;
        display_header(&term_obj.term, &path);

        let options = get_path_options(&path)?;

        let selection = Select::with_theme(&term_obj.theme)
            .items(&options)
            .default(0)
            .interact_on_opt(&term_obj.term)?;

        repl = match selection {
            Some(index) => handle_selection(&term_obj, &origin, &mut path, &options, index),
            None => {
                term_obj.term.write_line("User did not select anything")?;
                false
            }
        };
    }

    Ok(())
}
