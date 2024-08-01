use std::path::PathBuf;
use std::env;
use snafu::{ResultExt, Snafu};
use log::{LevelFilter,debug};

use graph;
use termprint as tp;

mod args;

mod filesystem;
use filesystem::FileSystem;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Build tree error"))]
    BuildError{source: filesystem::Error},
    #[snafu(display("File listing error"))]
    FileListError{source: filesystem::Error},
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn list_files_recursively(root_dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    filesystem::get_files(&mut files, root_dir);
    files
}

fn main() -> Result<()> {

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // args
    let args: Vec<String> = env::args().collect();
    let app = args::parse_cli_arguments();
    let matches = app.try_get_matches_from(args).unwrap_or_else(|e| {e.exit();});
    let max_level =  matches.get_one::<usize>("max");
    
    // List of files with path
    let root_dir = matches.get_one::<String>("path").expect("Argument \"Path\" required!");
    let files_list = list_files_recursively(&PathBuf::from(&root_dir));
    // let files_list_str = files_list.iter().filter_map(|p| p.to_str()).collect::<Vec<String>>().context(FileListSnafu)?;
    let files_list_str = files_list.iter().map(|p| p.display().to_string()).collect();
    
    let output = FileSystem::print_file_list(files_list_str, max_level, Some(&PathBuf::from(root_dir)));
    println!("{}",output);
    Ok(())
}

