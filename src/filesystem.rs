use std::{fmt,fs,io};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::path::PathBuf;
use std::io::{BufReader,Read};
use chrono::{DateTime};
use termprint as tp;
use bytes::Bytes;
use std::time::SystemTime;

// use time::{OffsetDateTime};

// use crate::graph::{Graph,GraphBuilder};
use graph::{Graph,GraphBuilder};


#[derive(Debug, Clone, Eq)]
pub struct FileContent {
    pub name: String,
    pub path: String,
    pub parent: Option<String>,
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
    pub fn new(path: &str, length:usize, content_type: ContentType) -> Self {
        FileContent {
            path: path.to_string(), 
            name: FileContent::basename(&path),
            parent: FileContent::parent_folder(&path),
            length: length,
            content_type: content_type,
            e_tag: None,
            modification_time: 0,
            access_time: 0,
        }
    }

    pub fn unfold(&self) -> HashSet<FileContent> {
        let mut fc_set = HashSet::<FileContent>::new();
        let fc = FileContent::new(&self.path, self.length, self.content_type);
        //println!("---- {} ----->",&fc.path);
        fc_set.insert(fc);

        let path = match self.path.chars().nth(0).unwrap() {
            '/' => {
                let fc = FileContent::new("/", 0, ContentType::DIRECTORY);
                //println!("{}",&fc.path);
                fc_set.insert(fc);
                let (_, path) = self.path.split_at(1);
                path
            },
            _ => &self.path,
        };
        let folders: Vec<&str> = path.split("/").collect();
        for n in  1..folders.len() {
            let new_path = folders[0..n].join("/");           
            let fc = FileContent::new(&new_path, 0, ContentType::DIRECTORY);
            //println!("{}",&fc.path);
            fc_set.insert(fc);
        }
        fc_set
    }

    pub fn basename(path: &str)-> String {
        match path.rsplit_once("/") {
            Some((_,basename)) => basename.to_string(),
            None => path.to_string(),
        }
    }

    pub fn parent_folder(path: &str) -> Option<String> {
        match path.rsplit_once("/") {
            Some((parent,_)) => Some(parent.to_string()),
            None => None
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
        tp::write_title(f, "\nFile Content");
        writeln!(f, "{}", &tp::info("Path: ", &self.path,Some(width)))?;
        writeln!(f, "{}", &tp::info("Type: ", &self.content_type.to_string(),Some(width)))?;
        writeln!(f, "{}", &tp::info("Name: ", &self.name,Some(width)))?;
        writeln!(f, "{}", &tp::info("Parent: ", &self.parent.clone().unwrap_or("".to_string()),Some(width)))?;
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
    pub list: HashSet::<FileContent>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem{list: HashSet::<FileContent>::new()}
    }

    pub fn add(&mut self, path: &str, length: usize, content_type: ContentType) {
        self.list.extend(FileContent::new(path,length,content_type).unfold());
    }

    pub fn from_str_list(&mut self, files: Vec<&str>) {
        for f in files {
            self.add(f,0,ContentType::FILE);
        }
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
                                    let mut fc = FileContent::new(&entry.path().to_str().unwrap(), metadata.len() as usize, ContentType::FILE);
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

    pub fn unfold(&mut self) {
        let mut list_dirs = HashSet::<FileContent>::new();
        for fc in self.list.iter() {
            list_dirs.extend(fc.unfold());
        }
        self.list.extend(list_dirs);
    }

    pub fn print_file_list(file_list: Vec<&str>, max: Option<&usize>) -> String {
        let mut files = FileSystem::new();
        files.from_str_list(file_list);
        files.unfold();
        let mut g = files.build_graph();
        g.find_sources(); 
        if let Some(max_level) = max {
            g.set_max_display_level(max_level);
        }
        format!("{}",g)
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
        for fc in fc_list.into_iter() {
            file_system.list.extend(fc.unfold())
        }
        file_system
    }
}

impl GraphBuilder<FileContent> for FileSystem {
    fn build_graph(&self) -> Graph<FileContent> {
        let mut g: Graph<FileContent> = Graph::new();
        for fc in self.list.clone(){
            let path = fc.path.clone();
            let name = fc.name.clone();
            g.add_node(&path, &name, fc);
        }
        for fc in self.list.clone() {
            if let Some(parent) = fc.parent.clone() {
                if let Err(e) = g.add_edge_byname(&parent, &fc.path){
                    print!("Error: {}", e);
                }
            }
        }
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