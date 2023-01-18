use std::fs::read_to_string;

fn main() {
    let hosts = read_to_string("/etc/hosts").unwrap();
    println!("{hosts}");
}
