use std::{path::{Path, PathBuf}, process::exit, str, io::{stdout, stdin, BufRead, Write}};
use crate::strings;

// Welcome to the file that holds *even* terrible code!

pub fn check(directory: &PathBuf, appendage: &str) -> bool {
    directory.to_owned().push(Path::new(appendage));
directory.exists() }


/*   Modified extract from Okeanos.
     warning.prompt.xx

     prompt_text     Asks for text input, returns String
     prompt_boolean  Asks for bool input, returns bool
*/// prompt_wait     Waits for any input, does nothing.

pub fn prompt_text(text: &String) -> String { loop {
    print!("{} :: [Y/n]: ", text); stdout().flush().unwrap();
    match stdin().lock().lines().next().unwrap() {
        Ok(content) => { match content.as_str() {
            "" => { eprintln!("{}", strings::fetch("warning.prompt.emptyInput")); continue; },
            text => return text.to_string() }},
        Err(error) => { eprintln!("{} :: {}", strings::fetch("errors.stdinFail"), error); exit(-1); },
}}}
pub fn prompt_boolean(text: &String) -> bool { loop {
    print!("{} :: [Y/n]: ", text); stdout().flush().unwrap();
    match stdin().lock().lines().next().unwrap() {
        Ok(content) => { match content.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
        "" => return true, _ => { eprintln!("{}", strings::fetch("warning.prompt.invalidChoice")); continue; }, }},
        Err(error) => { eprintln!("{} :: {}", strings::fetch("errors.stdinFail"), error); exit(-1); },
}}}
pub fn prompt_wait(text: &String) {
    print!("{}", text); stdout().flush().unwrap();
    match stdin().lock().lines().next().unwrap() {
        Ok(content) => { match content.as_str() { _ => (), }},
        Err(error) => { eprintln!("{} :: {}", strings::fetch("errors.stdinFail"), error); exit(-1); },
}}