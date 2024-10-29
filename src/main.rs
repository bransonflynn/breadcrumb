fn main() {

    crumbs::on_desktop();
}

pub mod crumbs {
    use std::{fs::File, io::{self}, path::Path};

    pub fn on_desktop() {
        std::println!("breadcrumb::main::crumbs::on_desktop -> started...");
        
        let computer_username: String = whoami::username();
        let target_path_string: String = "C:/Documents and Settings/".to_string() + &computer_username + "/Desktop/breadcrumb.txt";
        let target_path = Path::new(&target_path_string);
        let mut file = File::create(target_path).expect("on_desktop -> create failed");

        // the text to be written
        let text = String::from("Hello, World!");

        io::Write::write(&mut file, text.as_bytes()).expect("on_desktop -> write failed");
        std::println!("breadcrumb::main::crumbs::on_desktop -> finished!");
    }
}
