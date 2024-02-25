use std::{borrow::BorrowMut, collections::HashMap, fs, rc::{Rc, Weak}};

pub trait Sizeable {
    fn size(&self) -> u32;
}

#[derive(Debug, Clone, Default)]
struct Folder {
    files: HashMap<String, File>,
    subfolders: HashMap<String, Folder>,
    parent: Weak<Folder>
}

// #[derive(Debug)]
// struct Node {
//     folder: Folder,
//     parent: Weak<Node>,
//     children: HashMap<String, Node>,
// }

// impl Node {
//     fn empty() -> Node {
//         Node {
//             folder : Folder::new_empty(),
//             children: HashMap::new()
//         }
//     }

//     fn add_child(&mut self, name: &String, child: Node) {
//         self.children.insert(name.clone(), child);
//     }
// }

impl Folder {
    fn new(files: Option<HashMap<String, File>>, subfolders: Option<HashMap<String, Folder>>, parent: Option<Folder>) -> Folder {
        let files = files.unwrap_or_default();
        let subfolders = subfolders.unwrap_or_default();
        let parent = parent.unwrap_or_default();
        let parent = Rc::downgrade(&Rc::new(parent));
        // dbg!(&files);
        Folder {
            files,
            subfolders,
            parent,
        }
    }

    fn new_empty() -> Folder {
        let files: HashMap<String, File> = HashMap::new();
        let subfolders: HashMap<String, Folder>= HashMap::new();
        let parent = Weak::new();
        Folder {
            files,
            subfolders,
            parent,
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

    fn add_text_line(&mut self, line: &str) {
        if line.trim().starts_with("dir") {
            self.add_subfolder(line.trim().split_whitespace().nth(1).expect("should be there").to_string(), Folder::new_empty())
        } else {
            let mut words = line.trim().split_whitespace();
            let size: u32 = words.next().expect("should be there").parse().expect("should be there");
            let name = words.next().expect("should be there");
            self.add_file(name.to_string(), File {size});
        }
    }

    fn size(&self) -> u32 {
        self.files.values().fold(0, |a, b| a + b.size) + self.subfolders.values().fold(0,|a, b| a + b.size())
    }
}

#[derive(Debug, Clone)]
struct File {
    size: u32,
}


fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    parse(&contents);
}

fn parse(text: &String) {
    let mut root = Folder::new_empty();
    let mut cwd = &mut root;

    let blocks = text.split("$")
        .skip(1);

    for block in blocks {
        let mut lines = block.lines();
        let command_line = lines.next().expect("every block has a first line").trim();
        let mut command_line_words = command_line.split_whitespace();
        let cmd = command_line_words.next().expect("always here");
        let arg = command_line_words.next();
        
        let respones_lines = lines;

        match cmd {
            "cd" => {
                let arg = arg.expect("always arg for cd"); 
                cwd.add_subfolder(arg.to_string(), Folder::new_empty());
                cwd = cwd.subfolders.get_mut(arg).expect("Just added it");
            },
            
            "ls" => {
                for line in respones_lines {
                    cwd.add_text_line(line);
                }
            },

            _ => ()
        }
    }
    let size = root.size();
    dbg!(root);
    dbg!(size);
}
