use dirs::home_dir;

use crate::SystemPackage;
use std::fs;
use std::process::Command;

pub fn uninstall(package_name:&String,_:SystemPackage){
    let socks_home_path = home_dir().unwrap().join("devsocks");
    let socks_system_dir = socks_home_path.join("system");
    let socks_package_dir = socks_system_dir.join("packages");
    let socks_binary_dir = socks_system_dir.join("bin");
    fs::remove_dir_all(socks_package_dir.join(&package_name)).unwrap();
    fs::remove_file(socks_binary_dir.join(&package_name)).unwrap();
    println!("uninstalled {package_name}")
}

pub fn install(package_name:&String,package:SystemPackage){
    let socks_home_path = home_dir().unwrap().join("devsocks");
    let socks_system_dir = socks_home_path.join("system");
    let socks_package_dir = socks_system_dir.join("packages");
    let socks_binary_dir = socks_system_dir.join("bin");
    Command::new("git").args(vec![
        "clone".to_string(),
        format!("https://github.com/{}",package.repo),
        format!("{}",&socks_package_dir.join(&package_name).to_str().unwrap())
    ]).output().ok();
    Command::new("cargo").args(vec![
        "build".to_string(),
        "--release".to_string(),
        "--manifest-path".to_string(),
        format!("{}/Cargo.toml",&socks_package_dir.join(&package_name).to_str().unwrap()),
        "--target-dir".to_string(),
        format!("{}/{package_name}.build",&socks_binary_dir.to_str().unwrap())
    ]).output().ok();
    let out = Command::new("cargo").args(vec![
        "pkgid".to_string(),
        "--manifest-path".to_string(),
        format!("{}/Cargo.toml",&socks_package_dir.join(&package_name).to_str().unwrap()),
    ]).output().unwrap().stdout;
    let raw_package_id = String::from_utf8(out).unwrap_or("".to_string());
    let raw_package_id: Vec<&str> = raw_package_id.split("/").last().unwrap().split("#").collect();
    let raw_package_id: String = raw_package_id.last().unwrap().to_string();
    let raw_package_id: Vec<&str> = raw_package_id.split("@").collect();
    let raw_package_id = raw_package_id[0];
    fs::rename(format!("{}/{package_name}.build/release/{raw_package_id}",&socks_binary_dir.to_str().unwrap()), format!("{}/{package_name}",&socks_binary_dir.to_str().unwrap())).ok();
    fs::remove_dir_all(format!("{}/{package_name}.build/",&socks_binary_dir.to_str().unwrap())).ok();
    println!("{} installed as {}!",raw_package_id,&package_name);
    }