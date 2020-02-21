
pub use std::io;
pub use std::{env, fs};
pub use std::process::Command;

pub fn mac_git_matcher(path: &std::path::PathBuf, msg: &String) {
    let path_string: String = path.to_str().unwrap().to_owned();
    if let Ok(_) = Command::new("git").arg("-C").arg(&path_string).arg("add").arg(".").output() {
        if let Ok(_) = Command::new("git").arg("-C").arg(&path_string).arg("commit").arg("-m").arg(msg).output() {
            Command::new("git").arg("-C").arg(&path_string).arg("push").output().expect("Fail to push");
            () 
        }
    }
}

// Legacy
/*
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
*/