use std::process::Command;

fn main() {
    match Command::new("ls") //
        .arg("-l")
        .spawn()
    {
        Ok(..) => println!("okok"),
        Err(..) => println!("err"),
    }

    // println!("rest")
}
