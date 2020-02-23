#![allow(dead_code, non_snake_case)]
extern crate chrono;
extern crate ansi_term;

mod base;
mod gitmain;
mod flagcontrol;

use crate::base::*;
use crate::gitmain::*;
use crate::flagcontrol::chronoTime::*;

use std::io;
use std::{env, fs};
use std::path::Path;


fn main() ->io::Result<()> {
    // << Check if OS is macOS >>
    // Later use
    let os_key = os_checker();

    // Entry counter
    let mut COUNTER: usize = 0;
    
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

    // ----------------------------------------------------------------
    // Flag control
    // ----------------------------------------------------------------
    let flage_arg1 = chronoTime();

    // ----------------------------------------------------------------
    // main
    // ----------------------------------------------------------------
    let current_dir: std::path::PathBuf = env::current_dir()?; 
    for entry in fs::read_dir(&current_dir)? {
        let entry_in_string = entry.unwrap().path();
        // collect directories' info
        let metadata = fs::metadata(&entry_in_string)?;
        if metadata.is_dir() {
            print!("{:?}\n> Processing...", &entry_in_string);
            mac_git_matcher(&entry_in_string, &flage_arg1);
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
