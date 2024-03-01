use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum OldFsObject { 
    File(u32),
    Folder(HashMap<String, OldFsObject>)
}


#[derive(Debug)]
enum FsObject { 
    File(File),
    Folder(Folder)
}

#[derive(Debug, Default)]
struct Folder {
    name: String,
    size: Option<u32>,
    contents: HashMap<String, FsObject>
}

impl Folder {
    fn get_mut_nested_folder<'a>(&mut self, mut names: impl Iterator<Item=&'a String>) -> &mut Folder {
        match names.next() {
            None => self,
            Some(name) => {
                match self.contents.get_mut(name).expect("Looking for missing folder") {
                    FsObject::Folder(ref mut folder) => {
                        folder.get_mut_nested_folder(names)
                    }
                    _ => panic!("cd into a file"),
                }
            }
        }
    }

    fn from_name(name: String) -> Folder {
        return Folder {
            name, 
            size: None,
            contents: HashMap::new(),
        }
    }

    fn add_folder(&mut self, folder_to_add: Folder) {
        self.contents.insert(folder_to_add.name.clone(), FsObject::Folder(folder_to_add));
    }

    fn add_file(&mut self, file_to_add: File) {
        self.contents.insert(file_to_add.name.clone(), FsObject::File(file_to_add));
    }
}

#[derive(Debug, Default)]
struct File {
    name: String,
    size: u32,
}


fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let root = parse(&contents);
    let root = parse(&contents);
    let mut root_fs = FsObject::Folder(root);
    comput_folder_sizes(&mut root_fs);
    dbg!(&root_fs);
    let missing_space: u32 = 94000;
    println!("Missing space: {missing_space}");
    if let FsObject::Folder(root) = root_fs {
        let smol = find_smallest_folder_bigger_than(&root, missing_space);
        dbg!(&smol);
    }
    
}

fn comput_folder_sizes(root: &mut FsObject) -> &FsObject {
    if let FsObject::Folder(folder) = root {
        folder.size = Some(folder.contents.values_mut()
        .map(|f| comput_folder_sizes(f))
        .map(|f| {
            match f {
                FsObject::Folder(folder) => folder.size.expect("am i stoopid"),
                FsObject::File(file) => file.size
            }
        })
        .sum())
    }
    root
}

fn find_smallest_folder_bigger_than(root: &Folder, limit: u32) -> Option<&Folder> {
    root.contents.values()
    .filter_map(|e| match e {
        FsObject::Folder(f) => Some(f),
        _ => None,
    })
    .map(|f| find_smallest_folder_bigger_than(f, limit))
    .chain(std::iter::once(Some(root)))
    .fold(None, |a, b| {
        let best;
        dbg!(a, b);
        match (a, b) {
            (None, _) => best = b,
            (_, None) => best = a,
            (Some(f_a), Some(f_b)) => {
                if (f_a.size.expect("unwrapped") < limit) || (limit..f_a.size.expect("unwrapped")).contains(&f_b.size.expect("unwrapped")) {
                    best = b;
                } else {
                    best = a;
                }
            }
        };
        dbg!(best);
        println!();
        return best;
    })
}

// fn find_smallest_folder_bigger_than(root: &FsObject, limit: u32) -> &FsObject {
//     match root {
//         FsObject::File(_) => root,
//         FsObject::Folder(folder) => {
//             folder.contents.values()
//             .map(|f| find_smallest_folder_bigger_than(f, limit))
//             .fold(None, |a, b| {
//                 dbg!(a, b);
//                 println!();
//                 match a {
//                     None => Some(b),
//                     Some(a) => {
//                         match (a, b) {
//                             (FsObject::File(_), _) => Some(b),
//                             (FsObject::Folder(_), FsObject::File(_)) => Some(a),
//                             (FsObject::Folder(f_a), FsObject::Folder(f_b)) => {
//                                 if  f_a.size.expect("This should be called after they're resolved") < limit {
//                                     Some(b)
//                                 } else if (f_b.size < f_a.size) && (f_b.size > Some(limit)) {
//                                     Some(b)
//                                 } else {
//                                     Some(a)
//                                 }
//                             }
//                         }
//                     }
//                 }
//             })
//             .unwrap_or(root)
//         }
//     }
// }

fn parse(text: &String) -> Folder {
    let mut root = Folder::from_name("root".to_string());
    let mut current_dirs: Vec<String> = Vec::new();
    
    let blocks = text.split("$ ").skip(2);

    for block in blocks {
        let mut lines = block.lines();
        let command_line = lines.next().expect("every block has a first line").trim();
        let mut command_line_words = command_line.split_whitespace();
        let cmd = command_line_words.next().expect("always here");
        let arg = command_line_words.next();
        let respones_lines = lines;
        let cwd = root.get_mut_nested_folder(current_dirs.iter());

        match cmd {
            "cd" => {
                let arg = arg.expect("always arg for cd");
                if arg == ".." {
                    current_dirs.pop();
                } else if arg == "/" {
                    current_dirs = Vec::new();
                } else {
                    if !cwd.contents.contains_key(arg) {
                        cwd.add_folder(Folder::from_name(arg.to_string()));
                    }
                    current_dirs.push(arg.to_string());
                }
            },
            
            "ls" => {
                for line in respones_lines {
                    let trimmed_line = line.trim();
                    
                    if !trimmed_line.starts_with("dir") {
                        let mut words = trimmed_line.split_whitespace();
                        let size: u32 = words.next().unwrap().parse().expect("Std format expected");
                        let name = words.next().unwrap().to_string();
                        cwd.add_file(File{name, size})
                    }
                }
            },
            _ => ()
        }
    }
    root
}