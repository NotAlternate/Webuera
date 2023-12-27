use std::{path::{Path, PathBuf}, process::exit, env};
use webuera::*;

/*   Primary.rs :: Command line parser
---  Left hand side indicator

     Building:       Export to final product directory
*/// Initialisation: Generate for development.

fn main() {

    // Flags variable
    let mut confirm = false;
    let mut name = String::new();
    let mut _path = match env::current_dir() { Err(e) => {
        eprintln!("{} :: {}", strings::fetch("errors.pathRetrivialFail"), e); exit(-1); }
    Ok(a)=>a };

    let mut build_mode = false;
    let mut init_mode = false;

    // plap plap
    let args = env::args().collect::<Vec<String>>();
    let mut cmd = 0; match args.len() { 1 => { eprintln!("{}", strings::fetch("clp.insufficient")); exit(-1); },
        _ => { let mut index = 1; while index < args.len() { match args[index].as_str() {

            // Commands
            cmds if !cmds.starts_with("-") && cmd !=2=> { cmd=1; match cmds {
                "build" => build_mode = true,
                "init" => init_mode = true,
            _ => cmd=0 }}
 
/*  */      cmds if cmds.starts_with("-") && cmd !=2=> { match cmds {
/* cmds */      "-h" | "--help" => println!("{}", strings::fetch("commands.help")),
                "-v" | "--version" => println!("{}", strings::fetch("commands.version")),

                
/* flags */     "-y" | "--yes" => confirm=true,
                "-n" | "--name" => { index+=1; name = match index < args.len() {
                    true => { match args[index].to_owned() {
                        a if a.starts_with("-") => { eprintln!("{}", strings::fetch("clp.missingValue")); exit(-1); }
                        b => b
                }}, false => { eprintln!("{}", strings::fetch("clp.missingValue")); exit(-1); }}}

 /* pa */       "-pa" | "--pathAbsolute" => { index+=1; _path = match index < args.len() {
                    true => { match args[index].to_owned() {
                        a if a.starts_with("-") => { eprintln!("{}", strings::fetch("clp.missingValue")); exit(-1); }
                        b => { let new_path = Path::new(b.as_str()); match new_path.exists() {
                            false => { eprintln!("{}", strings::fetch("clp.invalidPath")); exit(-1); }
                        true => PathBuf::from(new_path) }}
                }}, false => { eprintln!("{}", strings::fetch("clp.missingValue")); exit(-1); }}}

/* pr */        "-pr" | "--pathRelative" => { index+=1; _path = match index < args.len() {
                    true => { match args[index].to_owned() {
                        a if a.starts_with("-") => { eprintln!("{}", strings::fetch("clp.missingValue")); exit(-1); }
                        b => { _path.push(Path::new(b.as_str())); match _path.exists() {
                            false => { eprintln!("{}", strings::fetch("clp.invalidPath")); exit(-1); }
                        true => _path }}
                }}, false => { eprintln!("{}", strings::fetch("clp.missingValue")); exit(-1); }}}
            _ => cmd=0 }}

    _ => cmd=0 } cmd = match cmd { 0=>2, o=>o }; index+=1; }}};

    // Checking of modes
    if !build_mode & !init_mode { eprintln!("{}", strings::fetch("clp.noJobs")); exit(-1); }

    println!("{} :: {}", build_mode, init_mode); // debug
    println!("{} :: {} :: {:?}", confirm, name, _path); // debug

}

/*
    // Checking of modes
    if !build_mode & !init_mode { eprintln!("{}", strings::fetch("clp.noJobs")); exit(-1); }


match vec!["webuera.config", "source/", "template/"].iter().all(|required| utility::check(&_path, required)) {
                    false => { eprintln!("{}", strings::fetch("build.")); exit(-1); }
                    true => { /* building process will take place in a different file */ }
                }


match vec!["webuera.config"].iter().all(|required| utility::check(&_path, required)) {
/* overwrite */     true => { match utility::prompt_boolean(&strings::fetch("init")) {
                        true => { }
                        false => { }
                    }}
/* empty */         false => { match utility::prompt_boolean(&strings::fetch("init")) {
                        true => { }
                        false => { }
                    }}
                }},
            _ => cmd=0 }
*/