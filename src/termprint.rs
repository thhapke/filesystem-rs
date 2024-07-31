use std::fmt;

use colored::Colorize;
use time::OffsetDateTime;

pub const LONG: usize = 120;
pub const MEDIUM: usize = 80;
pub const SHORT: usize = 30;


pub fn print_key_value(key: &str, value: &str, width1: Option<usize>) {
    let width: usize = width1.unwrap_or(key.len());
    println!("{:width$}: {}",key.blue(),value.cyan());
}

pub fn key_value(key: &str, value: &str, width1: Option<usize>) -> String {
    let width: usize = width1.unwrap_or(key.len());
    format!("{:width$}: {}",key.blue(),value.cyan())
}

pub fn error(message: &str) -> String {
    format!("ERROR: {}",message.red())
}

pub fn print_error(message: &str, info: Option<&str>, err_msg: Option<&str>) {
    if info.is_none() & err_msg.is_none() {
        println!("{} {}","ERROR ".to_string().bold().red(),message.red());
    }
    else {
        print!("{} {}","ERROR:".to_string().bold().red(),message.red());
    }
    if let Some(info_str) =  info {
        println!(": {}",info_str.red());
    }
    if let Some(err_str) =  err_msg {
        println!("{}",err_str.magenta());
    }
}

pub fn warning(message: &str) -> String {
    format!("Warning: {}",message.yellow())
}

pub fn print_warning(message: &str)  {
    println!("Warning: {}",message.yellow());
}

pub fn info(key: &str, value: &str, width1: Option<usize>) -> String {
    let width: usize = width1.unwrap_or(key.len());
    format!("{:width$}{}",key.blue(),value.cyan())
}

pub fn print_info(key: &str, value: &str, width1: Option<usize>) {
    let width: usize = width1.unwrap_or(key.len());
    println!("{:width$}{}",key.blue(),value.cyan());
}
pub fn write_info(f: &mut fmt::Formatter,key: &str, value: &str, width1: Option<usize>) {
    let width: usize = width1.unwrap_or(key.len());
    if let Err(e) =writeln!(f,"{:width$}{}",key.blue(),value.cyan()) { 
        print_error("Error write",Some(&"write_info".to_string()),Some(&e.to_string()))
    };
}

pub fn msg(msg: &str) -> String {
    format!("{}",msg.blue())
}

pub fn print_msg(msg: &str)  {
    println!("{}",msg.blue())
}

pub fn write_msg(f: &mut fmt::Formatter,msg: &str)  {
    writeln!(f,"{}",msg.blue()).unwrap()
}

pub fn print_title(msg: &str)  {
    println!("{}",msg.blue().bold())
}
pub fn write_title(f: &mut fmt::Formatter,msg: &str)  {
    writeln!(f,"{}",msg.blue().bold()).unwrap()
    
}
// pub fn print_info(key: &str, value: &str, width1: Option<usize>) -> String {
//     let width: usize = width1.unwrap_or(key.len());
//     println!("{:width$}: {}",key.blue(),value.cyan())
// }

pub fn print_line(length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    println!("{:⎯<width$}", "".cyan());
}

pub fn write_line(f: &mut fmt::Formatter,length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    if let Err(e) = writeln!(f,"{:⎯<width$}", "".cyan()) { 
        print_error("Error write",Some(&"write_line".to_string()),Some(&e.to_string()))
    };
}

pub fn print_double_line(length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    println!("{:═<width$}", "".cyan());
}

pub fn write_double_line(f: &mut fmt::Formatter,length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    if let Err(e) = writeln!(f,"{:═<width$}", "".cyan()) { 
        print_error("Error write",Some(&"write_double_line".to_string()),Some(&e.to_string()))
    };
}

pub fn print_map <T: std::fmt::Display> (title: &str,val: &str, map: T ) {
    print_line(Some(MEDIUM));
    print_key_value(title, val , None);
    print_line(Some(MEDIUM));
    println!("{}",map);
    print_line(Some(MEDIUM));
}



pub fn print_start_program(program_name: &str) -> OffsetDateTime {
    let local = OffsetDateTime::now_utc();
    println!("\n");
    print_double_line(Some(MEDIUM));
    println!("{} {}: {}",String::from("Start").blue(),program_name.bold().blue(),local.to_string().cyan());
    print_double_line(Some(MEDIUM));
    println!("\n");
    local
}

pub fn print_end_program(program_name: &str, start: OffsetDateTime) -> OffsetDateTime {
    let local = OffsetDateTime::now_utc();
    let duration = (local - start).to_string();
    println!("\n");
    print_double_line(Some(MEDIUM));
    println!("{} {}: {} - {}",String::from("End ").blue(),program_name.bold().blue(),
                              local.to_string().cyan(),duration.cyan().bold());
    print_double_line(Some(MEDIUM));
    println!("\n");
    local
}

pub enum TreeBlock {
    Item, 
    End
}

pub fn print_tree_item(item: &str,block_type: TreeBlock ) {
    match block_type {
        TreeBlock::Item => println!("{} {}","├──".blue(),item.cyan()),
        TreeBlock::End => println!("{} {}","└──".blue(),item.cyan()),
    }
}