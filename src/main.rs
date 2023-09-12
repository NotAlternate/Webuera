use std::{process::exit, env};
use konstruera::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut cmd = 0; match args.len() { 1 => (),
        _ => { let mut index = 1; while index < args.len() { match args[index].as_str() {
            cmds if cmds.starts_with("-") && cmd != 2 => { cmd=1; match cmds {
                "-h" | "--help" => println!("{}", strings::fetch("commands.help")),
                "-v" | "--version" => println!("{}", strings::fetch("commands.version")),
    _ => cmd=0 }} _ => cmd=0 } cmd = match cmd { 1=>exit(0), 0=>2, o=>o }; index+=1; }}};
}