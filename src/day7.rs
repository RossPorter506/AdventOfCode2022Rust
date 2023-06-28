use crate::prelude::*;

// Given a list of terminal outputs determine which folders are below a certain size, and return the sum of the size of these folders
pub fn calculate_part1() -> Result<usize>{
    const MAX_SIZE: usize = 100000;
    let file = File::open("input/day7.txt")?;
    let reader = BufReader::new(file);
    let mut current_filepath: Vec<String> = vec![];
    let mut root = FsElement::Directory("/".to_string(), vec![]);
    let mut current_directory: &mut FsElement = &mut root;

    let lines = reader.lines();

    for line in lines.flatten() {
        if line.is_empty() {continue;}

        match interpret_line(&line)? {
            LineContent::Command(ChangeDir(name)) => {
                // Update filepath
                match name {
                    "/" => current_filepath.clear(),
                    ".." => {current_filepath.pop();},
                    str => current_filepath.push(str.to_string()),
                };
                // Navigate to new directory
                current_directory = &mut root;
                for n in 0..current_filepath.len(){ 
                    let filepath = "/".to_string() + &current_filepath[..=n].join("/");
                    current_directory = current_directory.get_mut(&filepath)?;
                }
            },
            LineContent::Command(ListDir) => (),
            LineContent::DirectoryInfo(name) => {
                let filepath = if current_filepath.is_empty(){
                    "/".to_string() + name
                }
                else {
                    "/".to_string() + &current_filepath.join("/") + "/" + name
                };
                current_directory.add_if_new(FsElement::Directory(filepath, vec![]))?;
            },
            LineContent::FileInfo(name, size) => {
                let filepath = if current_filepath.is_empty(){
                    "/".to_string() + name
                }
                else {
                    "/".to_string() + &current_filepath.join("/") + "/" + name
                };
                current_directory.add_if_new(FsElement::File(filepath, size))?;
            },
        };
    }

    // We have a filesystem now.
    // All we need to do is iterate over all directories and get their sizes.
    let folder_list = walk_dir(&root);
    let total_size: usize = folder_list.into_iter()
        .map(|(_, size)| size)
        .filter(|size| size < &MAX_SIZE)
        .sum();
    Ok(total_size)
}

// Walk a folder and it's subfolders, returning a list of folders and their associated size.
fn walk_dir(dir: &FsElement) -> Vec<(&FsElement, usize)>{
    let mut folder_list: Vec<(&FsElement, usize)> = vec![];
    if let FsElement::Directory(_, contents) = dir {
        for dir in contents.iter().filter(|e| matches!(e, FsElement::Directory(_, _))) {
            folder_list.push((dir, dir.size()));
            folder_list.append(&mut walk_dir(dir));
        }
    }
    folder_list
}

#[derive(Debug)]
// Elements are either Directories or files.
// I'm not a fan of the runtime errors in this one, there's no good reason why you should be able to call 'add if new' or 'get' on a file, etc.
enum FsElement { 
    Directory(String, Vec<FsElement>),
    File(String, usize),
}
impl PartialEq for FsElement{
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}
impl FsElement {
    fn path(&self) -> &str {
        match self {
            FsElement::Directory(path, _) => path,
            FsElement::File(path, _) => path,
        }
    }
    fn size(&self) -> usize {
        match self {
            FsElement::Directory(_, contents) => {
                contents.iter().map(|e| e.size()).sum() // directory size is the sum of it's contents
            },
            FsElement::File(_, size) => *size,
        }
    }
    fn get(&self, name: &str) -> Result<&FsElement> {
        if let FsElement::Directory(_, contents) = self {
            for elem in contents {
                if elem.path() == name {
                    return Ok(elem);
                }
            }
            return Err(anyhow!("Element does not exist"));
        }
        Err(anyhow!("get called on file!"))
    }
    fn get_mut(&mut self, name: &str) -> Result<&mut FsElement> {
        if let FsElement::Directory(_, contents) = self {
            for elem in contents.iter_mut() {
                if elem.path() == name {
                    return Ok(elem);
                }
            }
            return Err(anyhow!("Element does not exist"));
        }
        Err(anyhow!("get mut called on file!"))
    }
    fn add_if_new(&mut self, elem: FsElement) -> Result<()>{
        if let FsElement::Directory(_, contents) = self {
            if !contents.contains(&elem) {
                contents.push(elem);
            }
            return Ok(());
        }
        Err(anyhow!("add called on a file"))
    }
}

// Enums for parsing terminal lines.
enum LineContent<'a> {
    Command(Command<'a>),
    DirectoryInfo(&'a str),
    FileInfo(&'a str, usize),
}

enum Command<'a> {
    ChangeDir(&'a str),
    ListDir
} use Command::*;

/// Take in one line of terminal input and determine what it represents
fn interpret_line(line: &str) -> Result<LineContent> {
    let first_char = line.chars().nth(0).unwrap(); // we checked it's non-empty already
    match first_char { 
        '$' => {
            let mut line = line.get(2..)
                .ok_or(anyhow!("Empty command after '$'."))?
                .split_ascii_whitespace();
            
            let cmd = line.next().unwrap(); // string is non-empty so this is guaranteed.
            let arg = line.next().ok_or(anyhow!("Missing argument to command!"));
            
            match cmd {
                "cd" => Ok( LineContent::Command(ChangeDir(arg?))),
                "ls" => Ok( LineContent::Command(ListDir) ),
                _ => Err(anyhow!("Invalid command!")),
            }
        },
        'd' => Ok(LineContent::DirectoryInfo(&line[4..])),
        '0' | '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' | '9' => {
            let mut line = line.split_ascii_whitespace();
            let size = line.next().unwrap().parse()?;
            let name = line.next().ok_or(anyhow!("Missing directory name after size!"))?;
            
            Ok(LineContent::FileInfo(name, size))
        }
        _ => Err(anyhow!("Invalid line!"))
    }
}

// Strip off the '$ ' before commands.
fn parse_command(line: &str) -> &str {
    &line[2..]
}

/***** Part 2 begins *****/

// Identical to above, except we do something slightly different near the bottom after we build the filesystem.
pub fn calculate_part2() -> Result<usize>{ 
    let file = File::open("input/day7.txt")?;
    let reader = BufReader::new(file);
    let mut current_filepath: Vec<String> = vec![];
    let mut root = FsElement::Directory("/".to_string(), vec![]);
    let mut current_directory: &mut FsElement = &mut root;

    let lines = reader.lines();

    for line in lines.flatten() {
        if line.is_empty() {continue;}

        match interpret_line(&line)? {
            LineContent::Command(ChangeDir(name)) => {
                match name {
                    "/" => current_filepath.clear(),
                    ".." => {current_filepath.pop();},
                    str => current_filepath.push(str.to_string()),
                };
                current_directory = &mut root;
                for n in 0..current_filepath.len(){ 
                    let filepath = "/".to_string() + &current_filepath[..=n].join("/");
                    current_directory = current_directory.get_mut(&filepath)?;
                }
            },
            LineContent::Command(ListDir) => (),
            LineContent::DirectoryInfo(name) => {
                let filepath = if current_filepath.is_empty(){
                    "/".to_string() + name
                }
                else {
                    "/".to_string() + &current_filepath.join("/") + "/" + name
                };
                current_directory.add_if_new(FsElement::Directory(filepath, vec![]))?;
            },
            LineContent::FileInfo(name, size) => {
                let filepath = if current_filepath.is_empty(){
                    "/".to_string() + name
                }
                else {
                    "/".to_string() + &current_filepath.join("/") + "/" + name
                };
                current_directory.add_if_new(FsElement::File(filepath, size))?;
            },
        };
    }

    // We have a filesystem now.
    // All we need to do is iterate over all directories and get their sizes.
    let folder_list = walk_dir(&root);

    const TOTAL_SPACE: usize = 70_000_000;
    const UPDATE_SPACE: usize = 30_000_000;

    let total_used_space: usize = root.size();
    let space_free = TOTAL_SPACE - total_used_space;
    let space_needed = UPDATE_SPACE - space_free;
    let mut potential_folders = folder_list.iter()
        .map(|(_, sz)| *sz)
        .filter(|sz| sz >= &space_needed)
        .collect::<Vec<usize>>();
    
    potential_folders.sort_unstable();

    Ok(potential_folders[0])
}