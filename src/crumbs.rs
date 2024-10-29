use std::{
    fs::File,
    io::{self},
    path::Path,
};
use whoami;

pub enum Directory {
    Desktop,
    Downloads,
}

pub fn put_crumbs(loc: Directory) {
    let target_path_string: String;
    let username = &whoami::username();

    match loc {
        Directory::Desktop => {
            target_path_string =
                "C:/Documents and Settings/".to_string() + username + "/Desktop/breadcrumb.txt";
        }
        Directory::Downloads => {
            target_path_string = "C:/Users/".to_string() + username + "/Downloads/breadcrumb.txt";
        }
    };

    let target_path: &Path = Path::new(&target_path_string);
    if Path::exists(&target_path) {
        std::println!("breadcrumb::crumbs::put_crumbs -> file already exists! return");
        return;
    }

    let mut file: File = File::create(target_path).expect("put_crumbs -> create failed");
    let text: String = String::from("Hello, World!");
    io::Write::write(&mut file, text.as_bytes()).expect("put_crumbs -> write failed");
    std::println!("breadcrumb::crumbs::put_crumbs -> finished!");
}
