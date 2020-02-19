#![allow(dead_code)]
extern crate chrono;

use std::io;
use std::{env, fs};
use chrono::prelude::*;
use std::process::Command;

fn os_checker() {
    if cfg!(target_os = "macos") {
        println!("<<< Your OS: macos >>>\nRunning Process..");
    } else if cfg!(target_os = "linux") {
        println!("<<< Your OS: linux >>>\nCan't sure it works on linux..");
    } else {
        panic!("You can't run this program in this os");
    }
}

fn git_matcher(path: &std::path::PathBuf, msg: &String) {
    match Command::new("cd").arg(path).output() {
        Ok(_) => {
            match Command::new("git").arg("add").arg("--a").output() {
                Ok(_) => {
                    match Command::new("git").arg("commit").arg("-m").arg(msg).output() {
                        Ok(_) => {
                            Command::new("git").arg("push").output().expect("Fail to push");
                            ()
                        },
                        Err(_) => {println!("Error in `git commit`"); ()},
                    }
                }
                Err(_) => {println!("Error in `git add`"); ()}
            }
        },
        Err(_) => {println!("Error in `cd`"); ()},
    }
}


fn main() ->io::Result<()> {
    os_checker();

    let local: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}", local);

    // std::result::Result<std::path::PathBuf, std::io::Error>
    // PathBuf or Error
    let current_dir = env::current_dir()?;
    println!("Current directory is: \n{:?}\n", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry_in_string = entry.unwrap().path();
        // println!("{:?}", entry_in_string);
        let metadata = fs::metadata(&entry_in_string)?;
        // println!("{:?}", metadata.is_dir());
        if metadata.is_file() {
            git_matcher(&entry_in_string, &local);
        }
    }

    /*
    let dir_list = 
        Command::new("sh")
            .arg("-c")
            .arg("ls > list.txt")
            .output()
            .expect("failed to execute `ls`");
    // dir_list.stdout;

    let output = if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("git log --oneline > hello.txt")
            // pub fn output(&mut self) -> Result<Output>
            .output()
            .expect("failed to execute process")
    } else {
        panic!("Only in MacOs")
    };
   // output.stdout;
   */

   Ok(())
}
