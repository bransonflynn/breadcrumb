use std::{
    fs::File,
    io::{self},
    path::Path,
};
use whoami;

pub fn on_desktop() {
    std::println!("breadcrumb::main::crumbs::on_desktop -> started...");

    let target_path_string: String =
        "C:/Documents and Settings/".to_string() + &whoami::username() + "/Desktop/breadcrumb.txt";
    let target_path: &Path = Path::new(&target_path_string);

    if Path::exists(&target_path) {
        std::println!("breadcrumb::main::crumbs::on_desktop -> file already exists! return");
        return;
    }

    let mut file: File = File::create(target_path).expect("on_desktop -> create failed");
    let text: String = String::from("Hello, World!");
    io::Write::write(&mut file, text.as_bytes()).expect("on_desktop -> write failed");
    std::println!("breadcrumb::main::crumbs::on_desktop -> finished!");
}
