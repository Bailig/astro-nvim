use ::std::process::Command;
use std::env::consts::OS;
use std::io::{stdin, stdout, Write};

fn main() {
    if OS != "macos" {
        println!("Only works on macos");
        return;
    }

    if !is_homebrew_installed() {
        print!("Do you want to install Homebrew? (y/n): ");

        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.eq_ignore_ascii_case("y") {
            install_homebrew();
        } else {
            return;
        }
    }

    println!("Homebrew is installed");
    println!("Installing neovim...");
    install_neovim();
    println!("Installing astronvim...");
    install_astronvim();
    println!("Installing astronvim config...");
    install_astronvim_config();
    println!("Installing nerd fonts...");
    install_nerd_fonts();
}

fn is_homebrew_installed() -> bool {
    let output = Command::new("brew").arg("--version").output().unwrap();
    output.status.success()
}

fn install_homebrew() {
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
        .output()
        .expect("Failed to install Homebrew");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn install_neovim() {
    let output = Command::new("brew")
        .arg("install")
        .arg("neovim")
        .output()
        .expect("Failed to install neovim");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn install_astronvim() {
    let output = Command::new("git")
        .arg("clone")
        .arg("--depth 1")
        .arg("https://github.com/AstroNvim/AstroNvim")
        .arg("~/.config/nvim")
        .output()
        .expect("Failed to install astronvim");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn install_astronvim_config() {
    let output = Command::new("git")
        .arg("clone")
        .arg("--depth 1")
        .arg("git@github.com:Bailig/astronvim-config.git")
        .arg("~/.config/nvim/lua/user")
        .output()
        .expect("Failed to install astronvim config");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn install_nerd_fonts() {
    let output = Command::new("brew")
        .arg("tap")
        .arg("homebrew/cask-fonts")
        .output()
        .expect("Failed to install nerd fonts");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg("font-hack-nerd-font")
        .output()
        .expect("Failed to install nerd fonts");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
