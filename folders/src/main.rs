use std::{collections::HashMap, fs};

pub trait Sizeable {
    fn size(&self) -> u32;
}

#[derive(Debug, Clone)]
struct Folder {
    files: HashMap<String, File>,
    subfolders: HashMap<String, Folder>,
}

// impl Sizeable for Folder {
//     fn size(&self) -> u32 {
//         self.files.iter().fold(0, |a, b| a + b.size) + self.subfolders.iter().fold(0,|a, b| a + b.size())
//     }
// }

impl Folder {
    fn new(files: Option<HashMap<String, File>>, subfolders: Option<HashMap<String, Folder>>) -> Folder {
        let files = files.unwrap_or_default();
        let subfolders = subfolders.unwrap_or_default();
        // dbg!(&files);
        Folder {
            files,
            subfolders,
        }
    }

    fn new_empty() -> Folder {
        let files: HashMap<String, File> = HashMap::new();
        let subfolders: HashMap<String, Folder>= HashMap::new();
        Folder {
            files,
            subfolders,
        }
    }

    fn merge(self, other: Folder) -> Folder {
        Folder {
            files : other.files.iter().chain(self.files.iter()).map(|(k, v)| (k.clone(), v.clone())).collect(),
            subfolders : other.subfolders.iter().chain(self.subfolders.iter()).map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }


    fn add_subfolder(&mut self, name: String, folder: Folder) {
        match self.subfolders.remove(&name) {
            None => self.subfolders.insert(name, folder),
            Some(f) => self.subfolders.insert(name, f.merge(folder))
        };

    }

    fn add_file(&mut self, name: String, file: File) {
        self.files.insert(name, file);
    }
}

#[derive(Debug, Clone)]
struct File {
    size: u32,
}

impl File {
    fn from_string(line: &str) -> File {
        println!("{line}");
        File { size : line.trim().split_whitespace().next().unwrap().parse().expect("ls output is fixed") }
        
    }
}


fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    parse(&contents);
}

fn parse(text: &String) {
    let mut root = Folder::new_empty();
    let mut cwd = root;

    text.split("$")
        .skip(1)
        .map(|block| {
            println!("Parsing block:\n{block}");
            let mut lines = block.lines();
            match lines.next() {
                None => (),
                Some(line) => {
                    let mut words = line.split_whitespace();
                    match words.next().unwrap() {
                        // "cd" => {
                        //     let name = words.next().unwrap().to_string();
                        //     cwd.add_subfolder(&name, Folder::new_empty());
                        //     cwd = cwd.subfolders.get_mut(&name).unwrap();
                        // },
                        // // "ls" => {let files: Vec<Files> = lines.map(|line| File::from_string(line)).collect();},
                        // "ls" => root.add_files(&mut lines.map(|line| File::from_string(line)).into_iter()),
                        _ => panic!(),
                    }
                }
            }
        })
        .collect::<Vec<_>>();
    // dbg!(root.size());
}
