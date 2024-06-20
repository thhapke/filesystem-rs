use std::path::PathBuf;
use std::env;

mod filesystem;
mod args;

use filesystem::FileSystem;

fn list_files_recursively(root_dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    filesystem::get_files(&mut files, root_dir);
    files
}

fn main() {

    // args
    let args: Vec<String> = env::args().collect();
    let app = args::parse_cli_arguments();
    let matches = app.try_get_matches_from(args).unwrap_or_else(|e| {e.exit();});
  
    // List of files with path
    let root_dir = matches.get_one::<String>("path").expect("Argument \"Path\" required!");
    let files_list = list_files_recursively(&PathBuf::from(&root_dir));
    let files_list_str = files_list.iter().filter_map(|p| p.to_str()).collect();
    
    
    let output = FileSystem::print_file_list(files_list_str);
    println!("{}",output);
}

