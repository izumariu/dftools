extern crate reqwest;
extern crate regex;
extern crate zip;

use regex::Regex;
use std::env;
use std::io::BufReader;
#[allow(unused_imports)] use std::process;

fn main() {

    let target_dir = get_target_dir();

    for font in &get_fontnames() {
        eprintln!("'{}' ==[to_urlname]=> {}", font, to_urlname(&font));
        let the_zip = get_zip(&font);
        let br = BufReader::new(the_zip);
    }

}


fn is_root() -> bool {

    match env::var("USER") {

        Ok(user) => (user == "root"),
        Err(_) => {
            eprintln!("E: COULDN'T FETCH ENV[USER]");
            false
        },

    }

}


fn get_target_dir() -> String {

    #[cfg(target_os = "linux")]
    let mut target_dir = String::from("share/fonts");

    #[cfg(target_os = "macos")]
    let mut target_dir = String::from("Library/Fonts");

    let mut final_dir = String::new();

    if is_root() {

        #[cfg(target_os = "linux")]
        target_dir.insert_str(0, "/usr/");

        #[cfg(target_os = "macos")]
        target_dir.insert_str(0, "/");

    } else {

        #[cfg(target_os = "linux")]
        final_dir.push_str(&format!("/home/{}/.local/", env::var("USER").unwrap()));

        #[cfg(target_os = "macos")]
        final_dir.push_str(&format!("/Users/{}/", env::var("USER").unwrap()));

    }

    final_dir.push_str(&target_dir);

    final_dir

}


fn get_fontnames() -> Vec<String> {

    let mut outvec:Vec<String> = Vec::new();

    let mut first_flag = true;

    for arg in env::args() {

        if first_flag {
            first_flag = false;
            continue;
        }

        outvec.push(arg);

    }

    outvec

}


fn to_urlname(before: &String) -> String {

    let re = Regex::new(r"\s+").unwrap();
    let lc = before.to_ascii_lowercase();
    let result = re.replace_all(lc.as_str(), "_");
    String::from(result)

}

fn get_zip(before: &String) -> String {
    let urlname = to_urlname(before);
    let body = reqwest::get(format!("https://dl.dafont.com/dl/?f={}", &urlname).as_str())
        .expect(&format!("FAIL#performGet @ GET {}", &urlname))
        .text()
        .expect(&format!("FAIL#extractBody @ GET {}", &urlname));
    body
}
