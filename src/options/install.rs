use std::process;
use std::io::Stdout;
use std::process::Child;

pub fn update() -> Child {
    std::process::Command::new("cargo")
        .args(["install", "mkpm"])
        .spawn()
        .unwrap()
}
