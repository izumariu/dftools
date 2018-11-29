extern crate reqwest;
extern crate regex;

use regex::Regex;
use std::env;
use std::process;

fn main() {

    if is_root() {
        eprintln!("You are root!");
    } else {
        eprintln!("You are not root.");
    }

    {

        let mut first_flag = true;

        for arg in env::args() {

            if first_flag {
                first_flag = false;
                continue;
            }

            println!("{}'' ==[to_urlname]=> {}", arg, to_urlname(&arg));
            
        }

    }

}


fn is_root() -> bool {

    match env::var("USER") {

        Ok(user) => (user == "root"),
        Err(_) => {
            eprintln!("COULDN'T FETCH ENV[USER]");
            process::exit(1);
        },

    }

}

fn to_urlname(before: &String) -> String { // TAKES OWNERSHIP!!

    let re = Regex::new(r"\s+").unwrap();
    let lc = before.to_ascii_lowercase();
    let result = re.replace_all(lc.as_str(), "_");
    let mut out = String::from(result);
    out.push_str(".font");
    out

}
