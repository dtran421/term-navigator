use std::{io::Error, path::PathBuf};

use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

const TITLE_STR: &str = "TERM-NAVIGATOR";
const PADDING: usize = 10;

pub struct TermObj {
    pub term: Term,
    pub theme: ColorfulTheme,
}

pub fn term_obj() -> TermObj {
    TermObj {
        term: Term::stderr(),
        theme: ColorfulTheme::default(),
    }
}

pub fn confirm_path(path: &PathBuf) -> bool {
    let pathname = path.to_str().expect("path string");

    let TermObj { term, theme } = term_obj();

    Confirm::with_theme(&theme)
        .with_prompt(format!("Confirm navigation to: {}?", &pathname))
        .interact_on(&term)
        .expect("confirm prompt displays and interactable")
}

pub fn display_select(options: &Vec<String>, results: usize) -> Result<Option<usize>, Error> {
    let TermObj { term, theme } = term_obj();

    FuzzySelect::with_theme(&theme)
        .with_prompt("🔎 Filter: ")
        .items(&options[..])
        .default(0)
        .max_length(results)
        .interact_on_opt(&term)
}

fn get_curr_dir(path: &PathBuf) -> String {
    let parent = path.parent().expect("path parent exists");
    let mut parent_str = parent.to_str().expect("parent string exists").to_owned();
    parent_str.push_str("/");

    let curr = path
        .file_stem()
        .expect("path file stem exists")
        .to_str()
        .expect("file stem to str");

    let formatted_path_str = format!(
        "🚀 The current directory is {}{}",
        console::style(parent_str).blue().dim(),
        console::style(curr).magenta().bold()
    );

    return formatted_path_str.as_str().to_string();
}

fn filler_line(total_len: usize, border_symbol: char, filler_symbol: String) -> String {
    format!(
        "{}{}{}",
        border_symbol,
        filler_symbol.repeat(total_len),
        border_symbol
    )
}

fn content_line(base_len: usize, content: &str) -> String {
    let spaces = " ".repeat(PADDING + (base_len - content.chars().count()) / 2);
    format!("|{}{}{}|", spaces, content, spaces)
}

pub fn display_header(origin: &PathBuf, path: &PathBuf) {
    let term = &term_obj().term;

    let orig_dir_str = format!(
        "Working Directory << {} >>",
        origin.to_str().expect("origin to string")
    );
    let base_len = orig_dir_str.chars().count();
    let total_len = base_len + 2 * PADDING;

    let border_line = filler_line(total_len, '*', "-".to_string());
    let spaces_line = filler_line(total_len, '|', " ".to_string());
    let equals_line = filler_line(total_len, '=', "=".to_string());

    let title_line = content_line(base_len, &TITLE_STR);
    let orig_dir_line = content_line(base_len, &orig_dir_str);
    let curr_dir_str = get_curr_dir(&path);

    term.write_line(&border_line).ok();
    term.write_line(&title_line).ok();
    term.write_line(&spaces_line).ok();
    term.write_line(&orig_dir_line).ok();
    term.write_line(&border_line).ok();

    term.write_line(&curr_dir_str).ok();
    term.write_line(&equals_line).ok();
}
