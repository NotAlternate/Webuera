use std::{process::exit, env};
use konstruera::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut cmd = 0; match args.len() { 1 => { eprintln!("{}", strings::fetch("clp.insufficient")); exit(-1); },
        _ => { let mut index = 1; while index < args.len() { match args[index].as_str() {
            // Commands
            cmds if cmd !=2=> { cmd=1; match cmds {
                "build" => { match vec!["konstruera.config", "source/", "template/"].iter().all(|required| utility::check(required)) {
                    false => { eprintln!("{}", strings::fetch("build.")); exit(-1); }
                    true => { /* building process will take place in a different file */ }
                }},
                "init" => { match vec!["konstruera.config", "source/", "template/"].iter().all(|required| utility::check(required)) {
                    true => { eprintln!("{}", strings::fetch("build.")); exit(-1); }
                    false => { /* itialisinad process will take in a diff file */ }
                }},
            _ => cmd=0 }}

            // Commands.. but with a twist.
            cmds if cmds.starts_with("-") && cmd !=2=> { cmd=1; match cmds {
                "-h" | "--help" => println!("{}", strings::fetch("commands.help")),
                "-v" | "--version" => println!("{}", strings::fetch("commands.version")),
    _ => cmd=0 }} _ => cmd=0 } cmd = match cmd { 1=>exit(0), 0=>2, o=>o }; index+=1; }}};
}