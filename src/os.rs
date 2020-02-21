
pub use ansi_term::Color::*;

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