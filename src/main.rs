/*     _                           _        
  __| | _____   _____  ___   ___| | _____ 
 / _` |/ _ \ \ / / __|/ _ \ / __| |/ / __|
| (_| |  __/\ V /\__ \ (_) | (__|   <\__ \
 \__,_|\___| \_/ |___/\___/ \___|_|\_\___/

 v: 0.1.0

 the package manager from dante
*/
const VERSION: &str = "0.1.0";

use std::{collections::HashMap, env::var, fs, process::{self, Stdio}};
use dirs::home_dir;
use toml::{de::from_str,toml};
use serde::{Deserialize, Serialize};
use clap::{Command,Arg,ArgMatches};
mod package;

#[derive(Serialize, Deserialize)]
struct SystemConf{
    devsocks:DevsocksConf,
    packages:HashMap<String, SystemPackage> 
}
#[derive(Serialize, Deserialize)]
struct SystemPackage{
    repo:String,
    version:String,
}
#[derive(Serialize, Deserialize)]
struct DevsocksConf{
    version:String,
}
fn main(){
    let shell = Command::new("devsocks").about("Package manager from Dante's Inferno").author("ichim8 <ichim8@icloud.com> : https://github.com/ichim9")
    .subcommand_required(true)
    .subcommand(Command::new("eval").about("Evaluate changes made to system.sox"))
    .subcommand(Command::new("init").about("Initializes devsocks setup"))
    .subcommand(Command::new("version").about("Prints version"))
    .subcommand(Command::new("config").about("Opens the system.sox in $EDITOR"))
    .subcommand(Command::new("integration").about("Prints the line necessary for $PATH").arg_required_else_help(true).arg(Arg::new("shell"))).get_matches();

    match shell.subcommand(){
        Some(("eval",_)) => {eval_devsock_config();}
        Some(("init",_)) => {init_devsock_config();}
        Some(("version",_)) => {version_devsock();}
        Some(("config",_)) => {config_edit();}
        Some(("integration",args)) => {integration(args);}
        _=>{}
    }
}

fn config_edit(){
    let socks_home_path = home_dir().unwrap().join("devsocks");
    let socks_system_path = socks_home_path.clone().join("system.sox");
    if fs::exists(&socks_system_path).unwrap_or(false){
        if let Ok(config_editor) = var("EDITOR"){
            let cmd_child = process::Command::new(config_editor)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg(socks_system_path.to_str().unwrap()).spawn();
            let _ = cmd_child.unwrap().wait();
        }else{
            let cmd_child = process::Command::new("vi")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg(socks_system_path.to_str().unwrap()).spawn();
            let _ = cmd_child.unwrap().wait();
        }
    }else{ 
        println!("system.sox doesn't exist!, run the init subcommand to fix this")
    }  
}

fn eval_devsock_config(){
    let socks_home_path = home_dir().unwrap().join("devsocks");
    let lock_file = socks_home_path.join("lock.sox");
    let socks_system_path = socks_home_path.clone().join("system.sox");
    if fs::exists(&socks_system_path).unwrap_or(false){
        let system_config = fs::read_to_string(&socks_system_path).unwrap_or("".to_string());
        if let Ok(json_data) = from_str::<SystemConf>(&system_config){
        if let Ok(previous_data) = from_str::<SystemConf>(&fs::read_to_string(&lock_file).unwrap_or("".to_string())){
        for (key,value) in json_data.packages{
            if !previous_data.packages.contains_key(&key){
                package::install(&key,value);
            }
        }
        for (key, value) in previous_data.packages{
            if !system_config.contains(&key){
                package::uninstall(&key,value);
            }
        }
        let _ = fs::write(lock_file,system_config);
    }else{
        println!("sox.lock is formatted incorrectly?");
        let json_data = from_str::<SystemConf>(&fs::read_to_string(&lock_file).unwrap_or("".to_string())).err().unwrap();
        println!("{}",json_data)
    }
    }else{
        println!("system.sox is formatted incorrectly?");
        let json_data = from_str::<SystemConf>(&system_config).err().unwrap();
        println!("{}",json_data)
    }
    }else{
        println!("system.sox doesn't exist!, run the init subcommand to fix this")
    }
}

fn version_devsock(){
    println!("devsock: v: {}",VERSION)
}
fn init_devsock_config(){
    let socks_home_path = home_dir().unwrap().join("devsocks");
    let socks_system_path = socks_home_path.clone().join("system.sox");
    let lock_file = socks_home_path.join("lock.sox");
    let socks_system_dir = socks_home_path.join("system");
    let socks_package_dir = socks_system_dir.join("packages");
    let socks_binary_dir = socks_system_dir.join("bin");
    fs::create_dir(socks_home_path).ok();
    fs::create_dir(socks_system_dir).ok();
    fs::create_dir(socks_package_dir).ok();
    fs::create_dir(socks_binary_dir).ok();
    let default_conf: toml::Table = toml!{
        [devsocks]
        version = VERSION
    
        [packages]
    };
    fs::write(&socks_system_path,default_conf.to_string()).ok();
    fs::write(&lock_file,default_conf.to_string()).ok();
}

fn integration(args:&ArgMatches){
    let shell = args.get_one::<String>("shell").unwrap();
    match shell{
        shell if shell ==  &"zsh".to_string()=>{
            println!("export PATH=\"~/devsocks/system/bin/:$PATH\"")
        }
        shell if shell == &"bash".to_string( )=>{
            println!("PATH=$PATH:~/devsocks/system/bin/")
        }
        shell if shell == &"fish".to_string()=>{
            println!("fish_add_path -p \"~/devsocks/system/bin\"")
        }
        shell if shell == &"nu".to_string()=>{
            println!("env.PATH = ($env.PATH | split row (char esep) | append \"~/devsocks/system/bin\")")
        }
        _=>{
            let shells = vec!["bash","zsh","fish","nu"];
            println!("Invalid shell!, valid shells include :\n{}",shells.join("\n"))
        }
    }
}