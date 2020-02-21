#![allow(dead_code, non_snake_case)]
extern crate chrono;
extern crate ansi_term;

mod os;
mod gitmain;

use crate::os::*;
use crate::gitmain::*;

use std::io;
use std::{env, fs};
use std::path::Path;
use chrono::prelude::*;


/*
fn os_checker() {
    if cfg!(target_os = "macos") {
        println!("{}\n{}", RGB(16, 200, 132).paint("<<< Your OS: macos >>>"), Red.paint("Running Process.."));
    } else if cfg!(target_os = "linux") {
        println!("<<< Your OS: linux >>>\nCan't sure it works on linux..");
    } else {
        panic!("You can't run this program in this os");
    }
}
*/
/*
fn mac_git_matcher(path: &std::path::PathBuf, msg: &String) {
    let path_string: String = path.to_str().unwrap().to_owned();
    match Command::new("git").arg("-C").arg(&path_string).arg("add").arg(".").output() {
        Ok(_) => {
            match Command::new("git").arg("-C").arg(&path_string).arg("commit").arg("-m").arg(msg).output() {
                Ok(_) => {
                    Command::new("git").arg("-C").arg(&path_string).arg("push").output().expect("Fail to push");
                    ()
                },
                Err(_) => {println!("Error in `git commit`"); ()},
            }
        }
        Err(_) => {println!("Error in `git add`"); ()}
    }
}
*/

fn main() ->io::Result<()> {
    // << Check if OS is macOS >>
    let os_key = os_checker();

    // Entry counter
    let mut COUNTER: usize = 0;

    // std::result::Result<std::path::PathBuf, std::io::Error>
    // PathBuf or Error
    
    // Argument parsing
    let args: Vec<String> = env::args().collect();
    let current_dir: String = args[1].parse().unwrap();
    match args.len() {
        // no argument
        1 => panic!("Lack of arguement"),
        2 => println!("Working directory is: \n{:?}\n", &current_dir),
        _ => panic!("More than 1 argument"),
    }

    // Set current dir to parsed dir
    let change_root_dir_to = Path::new(&current_dir);
    env::set_current_dir(&change_root_dir_to)?;

    // Get User's local time
    let local: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}", local);

    // main
    let current_dir: std::path::PathBuf = env::current_dir()?; 
    for entry in fs::read_dir(&current_dir)? {
        let entry_in_string = entry.unwrap().path();
        // collect directories' info
        let metadata = fs::metadata(&entry_in_string)?;
        if metadata.is_dir() {
            print!("{:?}\n> Processing...", &entry_in_string);
            mac_git_matcher(&entry_in_string, &local);
            println!("END");
            COUNTER = COUNTER+1;
        }
    }

    println!("\nRun through {} directories", COUNTER);
    println!("Done!");
    Ok(())
}

    // START DEBUG: 
    /*
    let current_dir2: std::path::PathBuf = env::current_dir()?;
    println!("----{:?}", &current_dir2);
    for entry in fs::read_dir(&current_dir2)? {
        println!("Is it path?>> {:?}", entry);
        println!("Is it path?>> {:?}", fs::metadata(entry.unwrap().path())?.is_dir());
    }
    */
    // END DEBUG
