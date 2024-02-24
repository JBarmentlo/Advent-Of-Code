use std::fs;

pub trait Sizeable {
    fn size(&self) -> u32;
}

enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    subfolders: Vec<Folder>,
}

impl Sizeable for Folder {
    fn size(&self) -> u32 {
        self.files.iter().fold(0, |a, b| a + b.size) + self.subfolders.iter().fold(0,|a, b| a + b.size())
    }
}

impl Folder {
    fn new(name: &str, files: Option<Vec<File>>, subfolders: Option<Vec<Folder>>) -> Folder {
        let name = String::from(name);
        let files = files.unwrap_or_default();
        let subfolders = subfolders.unwrap_or_default();
        // dbg!(&files);
        Folder {
            name,
            files,
            subfolders,
        }
    }

    fn new_empty(name: &str) -> Folder {
        let name = String::from(name);
        let files: Vec<File> = Vec::new();
        let subfolders: Vec<Folder> = Vec::new();
        Folder {
            name,
            files,
            subfolders,
        }
    }

    fn add_subfolder(&mut self, subfolder: Folder) {
        self.subfolders.push(subfolder);
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    
    fn add_files(&mut self, files: &mut dyn Iterator<Item = File>) {
        self.files.extend(files);
    }

}

#[derive(Debug, Clone)]
struct File {
    name: String,
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
    let mut root = Folder::new("root", None, None);
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
                            "cd" => {
                                cwd.add_subfolder(Folder::new_empty(words.next().unwrap()));
                                // cwd = cwd.ge
                            },
                            // "ls" => {let files: Vec<Files> = lines.map(|line| File::from_string(line)).collect();},
                            "ls" => root.add_files(&mut lines.map(|line| File::from_string(line)).into_iter()),
                            _ => panic!(),
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
    dbg!(root.size());
}
