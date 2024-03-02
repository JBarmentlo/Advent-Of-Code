use std::collections::HashMap;
use std::fs;
use std::process::exit;


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
    
    let mut root = parse(&contents);
    comput_folder_sizes(&mut root);
    // TODO: handle emtpy fs
    dbg!(&root);

    let sum = sum_directories_smaller_than(&root, 100000);
    println!("Sum: {sum}");
    
    let total_space = 70000000;
    let needed_space = 30000000;
    let available_space = total_space - root.size.expect("computed");
    if available_space > needed_space {
        println!("You have enough space");
        exit(0);
    }
    let missing_space: u32 = needed_space - available_space;
    println!("Missing space: {missing_space}");

    let smol = find_smallest_folder_bigger_than(&root, missing_space);
    dbg!(&smol);
}

fn comput_folder_sizes(root: &mut Folder) {
    root.size = root.contents.values_mut()
    .map(|f| match f {
        FsObject::Folder(ref mut folder) => {
            comput_folder_sizes(folder);
            folder.size
        },
        FsObject::File(file) => Some(file.size),
    })
    .sum()
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
        // dbg!(a, b);
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
        // dbg!(best);
        // println!();
        return best;
    })
}

fn sum_directories_smaller_than(root: &Folder, limit: u32) -> u32 {
    let lower_ranks = root.contents.values()
    .filter_map(|e| match e {
        FsObject::Folder(f) => Some(f),
        _ => None,
    })
    .map(|f| sum_directories_smaller_than(f, limit))
    .sum();

    if root.size.expect("computed") < limit {
        root.size.expect("computed") + lower_ranks
    } else {
        lower_ranks
    }
}


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

#[cfg(test)]
mod tests {
    // Import the necessary modules
    use std::fs::OpenOptions;
    use std::io::Write;

    // This test writes to a file
    #[test]
    fn test_file() {
        // Opens the file ferris.txt or creates one if it doesn't exist.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // Print "Ferris" 5 times.
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }

    // This test tries to write to the same file
    #[test]
    fn test_file_also() {
        // Opens the file ferris.txt or creates one if it doesn't exist.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // Print "Corro" 5 times.
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }
}