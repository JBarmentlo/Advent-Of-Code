use std::collections::HashMap;
use std::fs;

fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    parse(&contents);
}

fn parse(text: &String) {
    let mut folder_names: Vec<String> = Vec::new();
    let mut files: HashMap<String, u32> = HashMap::new();
    let blocks = text.split("$ ")
        .skip(2);

    for block in blocks {
        let mut lines = block.lines();
        let command_line = lines.next().expect("every block has a first line").trim();
        let mut command_line_words = command_line.split_whitespace();
        let cmd = command_line_words.next().expect("always here");
        let arg = command_line_words.next();
        let respones_lines = lines;

        println!("{command_line}");
        match cmd {
            "cd" => {
                let arg = arg.expect("always arg for cd");
                if arg == ".." {
                    folder_names.pop();
                } else if arg == "/" {
                    folder_names = Vec::new();
                } else {
                    folder_names.push(arg.to_string());
                }

                let cwd = folder_names.join("/");
                println!("current_folder: {cwd}");
            },
            
            "ls" => {
                let cwd = folder_names.join("/");
                println!("current_folder: {cwd}");
                for line in respones_lines {
                    println!("\t{line}");
                    let trimmed_line = line.trim();
                    if  !trimmed_line.starts_with("dir") {
                        let mut words = trimmed_line.split_whitespace();
                        let file_size: u32 = words.next().unwrap().parse().expect("Std format expected");
                        let file_name = words.next().unwrap();
                        folder_names.push(file_name.to_string());
                        let full_path = folder_names.join("/");
                        folder_names.pop();
                        println!("\t\tAdd: {full_path}: {file_size}");
                        files.insert(full_path, file_size);
                    }
                }
            },

            _ => ()
        }
        println!();
        println!();
    }
    dbg!(&files);
    let total_size: u32 = files.values().sum();
    dbg!(total_size);
    parse_file_map(files);
}

enum FsContent {
    File(u32),
    Folder(HashMap<String, FsContent>)
}

fn parse_file_map(files: HashMap<String, u32>){
    // Folder(files.iter().filter)
    let mut keys: Vec<&String> = files.keys().collect();
    keys.sort();
    for key in keys {
        let value = files.get(key).unwrap();
        let count = key.split("/").count() - 1;
        println!("{count} : {key} : {value}");


    }

}