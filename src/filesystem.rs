
use std::{fmt,fs,io};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::path::PathBuf;
use std::io::{BufReader,Read};
use chrono::{DateTime};
// use termprint as tp;
use bytes::Bytes;
use std::time::SystemTime;
use snafu::Snafu;
use colored::Colorize;

use log::debug;


use graph::{Graph,GraphBuilder};
use termprint as tp;

pub const SHORT: usize = 30;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Root directory does not match with path: {} -> {}", root,path))]
    NoRootPath{root:String, path:String},
}

type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Debug, Clone, Eq)]
pub struct FileContent {
    pub name: String,
    pub path: PathBuf,
    pub parent: Option<PathBuf>,
    pub length: usize,
    pub content_type: ContentType,
    pub e_tag: Option<String>,
    pub modification_time: i64,
    pub access_time: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContentType {
    DIRECTORY,
    FILE,
    UNKNOWN,
}

impl ContentType {
    pub fn from(content_type: &str) -> ContentType {
        match content_type {
            "DIRECTORY" => ContentType::DIRECTORY,
            "FILE" => ContentType::FILE,
            _ => ContentType::UNKNOWN,
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self  {
            ContentType::DIRECTORY => write!(f, "DIRECTORY"),
            ContentType::FILE => write!(f, "FILE"),
            ContentType::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}

impl FileContent {
    pub fn new(path: &PathBuf, parent: Option<PathBuf>,length:usize, content_type: ContentType) -> Self {
        FileContent {
            path: path.clone(), 
            name: FileContent::get_name(path),
            parent: parent,
            length: length,
            content_type: content_type,
            e_tag: None,
            modification_time: 0,
            access_time: 0,
        }
    }

    pub fn get_name(path: &PathBuf) -> String {
        match path.file_name() {
            None => path.to_string_lossy().to_string(),
            Some(p) => p.to_string_lossy().to_string(),
        }
    }
}

impl PartialEq for FileContent {
    fn eq(&self, other: &FileContent) -> bool {
        self.path == other.path
    }
}

impl Hash for FileContent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl fmt::Display for FileContent {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //     write!(f, "{} ({}): parent: {} - {} - {}", self.name, self.path, self.parent.as_deref().unwrap_or("/"), self.length, self.content_type)
    // }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width:usize = 20;
        tp::write_title(f, "\nFile Content")?;
        writeln!(f, "{}", &tp::info("Path: ", &self.path.to_string_lossy(),Some(width)))?;
        writeln!(f, "{}", &tp::info("Type: ", &self.content_type.to_string(),Some(width)))?;
        writeln!(f, "{}", &tp::info("Name: ", &self.name,Some(width)))?;
        let parent = &self.parent.as_ref().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|| "".to_string());
        writeln!(f, "{}", &tp::info("Parent: ", parent,Some(width)))?;
        writeln!(f, "{}", &tp::info("Length: ", &self.length.to_string(),Some(width)))?;
        writeln!(f, "{}", &tp::info("eTag: ", &self.e_tag.clone().unwrap_or("".to_string()),Some(width)))?;
        let dt = DateTime::from_timestamp(self.modification_time/1000, ((self.modification_time % 1_000) * 1_000_000 )as u32);
        let dta = DateTime::from_timestamp(self.access_time/1000, ((self.access_time % 1_000) * 1_000_000) as u32);
        // let dt = OffsetDateTime::from_unix_timestamp(self.modification_time/1000).unwrap();
        if let Some(dtime) = dt {
            writeln!(f, "{}", &tp::info("Modification time: ",&dtime.to_string(),Some(width)))?;
        };
        if let Some(dtime) = dta {
        // let dta = OffsetDateTime::from_unix_timestamp(self.modification_time/1000).unwrap();
            write!(f, "{}", &tp::info("Access time: ",&dtime.to_string(),Some(width)))?;
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FileSystem  {
    pub root: Option<PathBuf>,
    pub list: HashSet::<FileContent>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem{
            root: None,
            list: HashSet::<FileContent>::new()
        }
    }

    pub fn set_root(&mut self,path: &PathBuf) {
        self.root = Some(path.clone());
        self.list.insert(FileContent::new(path,None, 0,ContentType::DIRECTORY));
    }

    pub fn add(&mut self, path: &PathBuf, length: usize, content_type: ContentType) -> bool {
        // test if path is root then do not add
        if let Some(r) = &self.root {
            if r == path {
                return false;
            }
        }
        let parent = path.parent().map(PathBuf::from);
        let fc = FileContent::new(path,parent.clone(), length,content_type);
        self.list.insert(fc);
        // unfolding
        if let Some(ppath) = parent { 
            self.add(&ppath, 0, ContentType::DIRECTORY);
        }
        true
    }

    pub fn from_str_list(&mut self, files: Vec<String>, root: Option<&PathBuf>)  {
        debug!("Build Filesystem data structure");
        let start_time = std::time::Instant::now();
        match root {
            Some(r) => {
                self.set_root(r);
                debug!("Set root: {:?}",r)
            },   
            None => {
                let mut common_root = PathBuf::from(files[0].clone());

                for path_str in &files[1..] {
                    let path = PathBuf::from(path_str);
                    // Truncate common_root to the common prefix with the current path
                    common_root = common_root
                        .components()
                        .zip(path.components())
                        .take_while(|(a, b)| a == b)
                        .map(|(a, _)| a)
                        .collect();
                    // If the common root becomes empty, there is no common path
                    if common_root.as_os_str().is_empty() {
                        break;
                    }
                }
                match common_root.as_os_str().is_empty() {
                    true => {
                        debug!("No root");
                        self.root = None;
                    },
                    false => {
                        self.set_root(&common_root);
                        debug!("Root path: {}",&common_root.to_string_lossy());
                    },
                }
            }
        };
        for f in files {
            self.add(&PathBuf::from(f),0,ContentType::FILE);
        };
        debug!("-> Elapsed Time: {:?} for #files: {}",start_time.elapsed(),self.list.len());
    }

    pub fn get_local_files(&mut self, root: &PathBuf) {
        let entries = match fs::read_dir(root) {
            Err(e) => {println!("Error reading folder. ({})",e.to_string()); return },
            Ok(f) => f,
        };
    
        for dir_entry in entries {
            match dir_entry {
                Err(e) => {println!("Error reading folder. ({})",e.to_string()); return },
                Ok(entry) => {
                    let file_type = entry.file_type();
                    match file_type {
                        Err(e) => {println!("Error file type: {:?} ({})",entry.path(),e.to_string()); continue}
                        Ok(file_type) => {
                            if file_type.is_file() {
                                if let Ok(metadata) = fs::metadata(&entry.path()) {
                                    let path = entry.path();
                                    let mut fc = FileContent::new(&path,path.parent().map(PathBuf::from), metadata.len() as usize, ContentType::FILE);
                                    fc.access_time = (metadata.accessed().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() * 1000) as i64;
                                    fc.modification_time = (metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() * 1000) as i64;
                                    self.list.insert(fc);
                                }
                            } else if file_type.is_dir() { // sym_links are excluded
                                self.get_local_files(&entry.path());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn print_file_list(file_list: Vec<String>, max: Option<&usize>, root:Option<&PathBuf>) -> String {
        let mut files = FileSystem::new();
        files.from_str_list(file_list, root);
        match files.root.clone() {
            None => format!("No root for printing as tree!"),
            Some(r) => {
                let mut g = files.build_graph();
                let root_name = r.to_string_lossy().to_string();
                match g.byname.get(&root_name) {
                    None => format!("Root node not found!"),
                    Some(rnode) => {
                        g.nodes[rnode.clone()].label = root_name.clone();
                        g.add_sources(rnode.clone());
                        if let Some(max_level) = max {
                            g.set_max_display_level(max_level);
                        }
                        let summary = format!("{:═<SHORT$}\n{} {}\n{:═<SHORT$}", "".blue(),"#files:".blue(),files.list.len().to_string().cyan(),"".blue());
                        format!("{}\n{}",g,summary)
                    },
                }
            },
        }
    }

}

// In src/filesystem.rs
impl IntoIterator for FileSystem {
    type Item = FileContent;
    type IntoIter = std::collections::hash_set::IntoIter<FileContent>;
    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl From<Vec<FileContent>> for FileSystem {
    fn from(fc_list: Vec<FileContent>) -> Self {
        let mut file_system = FileSystem::new();
        file_system.list.extend(fc_list);
        file_system
    }
}

impl GraphBuilder<FileContent> for FileSystem {
    fn build_graph(&self) -> Graph<FileContent> {
        debug!("Build Graph");
        let start_time = std::time::Instant::now();

        let mut g: Graph<FileContent> = Graph::new();
        for fc in self.list.clone(){
            let path = fc.path.clone();
            let name = fc.name.clone();
            g.add_node(&path.to_string_lossy().to_string(), &name, fc);
        }
        for fc in self.list.clone() {
            if let Some(parent) = fc.parent.clone() {
                if let Err(e) = g.add_edge_byname(&parent.to_string_lossy().to_string(), &fc.path.to_string_lossy().to_string()){
                    print!("Error: {}", e);
                }
            }
        }
        debug!("- Elapsed Time: {:?} for #nodes: {}",start_time.elapsed(),g.nodes.len());
        g
    }
}


pub fn get_files(files: &mut Vec<PathBuf>,folder_path: &PathBuf) {
    
    // Read all entries in the folder
    let entries = match fs::read_dir(folder_path) {
        Err(e) => {println!("Error reading folder. ({})",e.to_string()); return },
        Ok(f) => f,
    };

    for dir_entry in entries {
        match dir_entry {
            Err(e) => {println!("Error reading folder. ({})",e.to_string()); return },
            Ok(entry) => {
                let file_type = entry.file_type();
                match file_type {
                    Err(e) => {println!("Error file type: {:?} ({})",entry.path(),e.to_string()); continue}
                    Ok(file_type) => {
                        if file_type.is_file() {
                            files.push(entry.path());
                        } else if file_type.is_dir() { // sym_links are excluded
                            get_files(files, &entry.path());
                        }
                    }
                }
            }
        }
    }
}

pub fn data_volume_str(num_bytes: usize) -> String {
    match num_bytes {
        x if x > 1073742000 => format!("{} GB",num_bytes/1073742000),
        x if x > 1048576 => format!("{} MB",num_bytes/1048576),
        x if x > 1024 => format!("{} kB",num_bytes/1024), 
        _ => format!("{} Byte",num_bytes),
    }
}

pub fn get_file_bytes(file: &PathBuf) -> Result<Bytes, io::Error> {
    match fs::File::open(file) {
        Err(e) => Err(e),
        Ok(file) => {
            let mut buf = Vec::new();
            let mut reader = BufReader::new(file);
            match reader.read_to_end(&mut buf) {
                Err(e) => Err(e),
                Ok(_) => Ok(Bytes::from(buf))
            }
        }
    }
}