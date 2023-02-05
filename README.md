# Terminal Navigator

A simple, light-weight CLI utility for navigating through your filesystem via the terminal, built in Rust.

## Installation

To install the utility, download the latest release and place it somewhere convenient for you. Then, place the following code
in your shell config (`.zshrc`, `.bashrc`, or other).

```sh
run_term_navigator ()
{
    if [[ "$1" == "-h" || "$1" == "--help" ]]; then
        /Users/dtran/GitHub/term-navigator/target/release/term-navigator "$1"
    else
        dir=$(/Users/dtran/GitHub/term-navigator/target/release/term-navigator $@)
        cd "$dir"
    fi
}

alias term_navigator=run_term_navigator
```

## TODO

[ ] Integrate Text User Interface (TUI) like [Cursive](https://crates.io/crates/cursive)
