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

    // fn merge(&mut self, other: Option<&Folder>) {
    //     match other {
    //         None => (),
    //         Some(f) => {
    //             f.subfolders.iter().map(|(name, folder)| self.subfolders.entry(name.clone()).and_modify(|my_folder| my_folder.merge(Some(folder))).or_insert(*folder));
    //         }
    //     }
    // }

    fn merge(&self, other: &Folder) -> Folder {
        let merged_subfolders: HashMap<String, Folder> = self.subfolders.iter().map(|(name, folder)| {
            match other.subfolders.get(name) {
                None => (name.clone(), folder.clone()),
                Some(other_folder) => (name.clone(), Self::merge(folder, other_folder))
            }
        })
        .collect();

        let merged_files: HashMap<String, File> = self.files.drain().chain(other.files.drain()).collect();
        Folder { 
            files: merged_files, 
            subfolders: merged_subfolders 
        }
    }


    fn add_subfolder(&mut self, name: String, folder: Folder) {
        self.subfolders.insert(name, folder);
        // self.subfolders.entry(name).and_modify(|dir| dir.merge(folder))
        // TODO: merge shit
    }

    fn add_file(&mut self, name: String, file: File) {
        self.files.insert(name, file);
    }
    
    // fn add_files(&mut self, files: &mut dyn Iterator<Item = File>) {
    //     self.files.extend(files);
    // }

}

#[derive(Debug, Clone)]
struct File {
    size: u32,
}

impl File {
    fn from_string(line: &str) -> File {
        println!("{line}");
        File {
            size : line.trim().split_whitespace().next().unwrap().parse().expect("ls output is fixed"),
            name : line.trim().split_whitespace().nth(1).unwrap().to_string(),
        }
        
    }
}


fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    parse(&contents);
}

fn parse(text: &String) {
    let root = &mut Folder::new_empty();
    let mut cwd = root;
    let _ = text.split("$")
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
