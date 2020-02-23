
pub use clap::{Arg, App};
pub use ansi_term::Color::*;

// Os Checking
pub fn os_checker() ->u8{
    if cfg!(target_os = "macos") {
        println!("{}\n{}", RGB(16, 200, 132).paint("<<< Your OS: macos >>>"), Red.paint("Running Process.."));
        0
    } else if cfg!(target_os = "linux") {
        println!("<<< Your OS: linux >>>\nCan't sure it works on linux..");
        1
    } else {
        panic!("You can't run this program in this os");
        //2
    }
}

// Command Line Parsing
pub fn parse_cli() {
    let matches = App::new("git-pusher")
        .version("0.1.0")
        .author("HansBlackCat <https://github.com/HansBlackCat/git-pusher>")
        .about("parse directory and git add, commit, push whole subdirectory")
        .arg(Arg::with_name("chrono time")
                 .short("t")
                 .long("time")
                 .takes_value(true)
                 .help("Commit message base on your local time"))
        /*
        .arg(Arg::with_name("utc time")
                 .short("u")
                 .long("utc-time")
                 .takes_value(true)
                 .help("Commit message base on your local time"))
        */
        .get_matches();
    
    let matches_chrono_time = matches.value_of("chrono time");
    println!("Working directory is: \n{:?}\n", matches_chrono_time.unwrap_or("Wrong-directory!!"));
    let directory: String = match matches_chrono_time {
        None      => {
            println!("Need directory"); 
            panic!("Need Directory!")
        },
        Some(dir) => dir.to_owned(),
    };


    /*
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
    */
}