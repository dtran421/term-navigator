use clap::Parser;
use display::{term_obj, TermObj};
use regex::Regex;
use std::{env, io, path::PathBuf};

mod display;
mod navigator;

#[derive(Parser, Debug)]
#[command(
    author = "Duke Tran",
    version,
    about = "Terminal Navigator",
    long_about = "A simple CLI utility for navigating through your filesystem via the terminal."
)]
struct Args {
    /// List results without indexing.
    #[arg(short = 'n', long = "no-index")]
    no_index: bool,

    /// Show all folders, including hidden ones.
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Number of results to display per page.
    #[arg(short = 'r', long = "results", default_value_t = 10)]
    results: usize,

    /// Force navigate to folder (skip confirmation).
    #[arg(short = 'f', long = "force")]
    force: bool,

    /// Simple appearance. Hide header and formatting.
    #[arg(short = 's', long = "simple")]
    simple: bool,
}

fn handle_selection(
    origin: &PathBuf,
    path: &mut PathBuf,
    options: &Vec<String>,
    index: usize,
    force: bool,
) -> bool {
    match index {
        0 => {
            if path.to_str().eq(&origin.to_str()) {
                return false;
            }

            let confirm = match force {
                true => true,
                false => display::confirm_path(&path),
            };
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
            let re = Regex::new(r"^\[[0-9]+\] ").unwrap();
            let result = re.replace(options[idx].as_str(), "");
            path.push(result.to_string());
        }
    };

    return true;
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let Args {
        no_index,
        all,
        results,
        force,
        simple,
    } = args;

    let origin = env::current_dir()?;
    let mut path = env::current_dir()?;

    let TermObj { term, .. } = term_obj();

    let mut repl = true;
    while repl {
        term.clear_screen()?;
        if !simple {
            display::display_header(&origin, &path);
        }

        let options = navigator::get_path_options(&path, simple || no_index, all)?;

        let selection = display::display_select(&options, results)?;

        repl = match selection {
            Some(index) => handle_selection(&origin, &mut path, &options, index, force),
            None => {
                term.clear_screen()?;
                false
            }
        };
    }

    Ok(())
}
