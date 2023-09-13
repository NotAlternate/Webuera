use std::process::exit;
const VERSION: &str = include_str!("../VERSION");

// Best reason to not collaborate with me: this.

pub fn fetch(text: &str) -> String { let identifier=text.split('.').collect::<Vec<&str>>();
    match identifier.len() { 2|3=>(), _ => { eprintln!("Passed value has invalid identifier count."); exit(-1); }}
    let n=String::new(); let u=""; let mut c=false;
    /*   One line variables above are used for duct taping purposes.
    -->  Identifier[0]  Categories with/without symbols
         Identifier[1]  Further categories
                        * Non-existent sometimes
    */// Identifier[2]  Actual string name
    let final_=match identifier[0] {
/*  */  "clp" => format!("\x1b[1;31mxx\x1b[0m {}", match identifier[1] {
            "insufficient"=>"Insufficent number of parametres has been passed.",
        _=>{c=true;u}}),
/*  */  "commands" => match identifier[1] {
            "help"=>format!("\x1b[1mKonstruera {}\x1b[0m\nSite \x1b[4mbuilder\x1b[0m..?\n\nProgram parameters:\n    \x1b[1mKonstruera\x1b[0m [FLAGS]\n\nProgram flags:\n    \x1b[1m--help\x1b[0m & \x1b[1m-h\x1b[0m  ::  Shows Konstruera's description and usage.\n    \x1b[1m--version\x1b[0m & \x1b[1m-v\x1b[0m  ::  Outputs current version of Konstruera.", VERSION),
            "version"=>VERSION.to_string(),
        _=>n },
/*  */  "warning" => format!("\x1b[1;33m!!\x1b[0m {}", match identifier[1] {

        _=>{c=true;u}}),
/*  */  "errors" => format!("\x1b[1;31mxx\x1b[0m {}", match identifier[1] {

        _=>{c=true;u}}),
    _=>n }; if final_.is_empty() || c { eprintln!("No matches found for `{}`", text); exit(-1); } else {
final_ }}