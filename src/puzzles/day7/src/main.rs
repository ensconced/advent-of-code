use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct DirInfo<'a> {
    size: u32,
    entries: HashMap<String, DirEntry<'a>>,
    parent: Option<Rc<RefCell<&'a DirInfo<'a>>>>,
}

enum DirEntry<'a> {
    file { size: u32 },
    dir(DirInfo<'a>),
}

impl<'a> DirEntry<'a> {
    fn new_child_dir(parent: Rc<RefCell<&'a DirInfo<'a>>>) -> Self {
        Self::dir(DirInfo {
            size: 0,
            entries: HashMap::new(),
            parent: Some(parent),
        })
    }
}

enum ChangeDirDest<'a> {
    Up,
    Root,
    ChildDir { dir_name: &'a str },
}

enum ParsedLine<'a> {
    ChangeDir(ChangeDirDest<'a>),
    ListDirs,
    Directory { dir_name: &'a str },
    File { file_size: u32 },
}

fn parse_line(line: &str) -> ParsedLine {
    if line.starts_with("$ cd /") {
        ParsedLine::ChangeDir(ChangeDirDest::Root)
    } else if line.starts_with("$ cd ..") {
        ParsedLine::ChangeDir(ChangeDirDest::Up)
    } else if line.starts_with("$ cd ") {
        ParsedLine::ChangeDir(ChangeDirDest::ChildDir {
            dir_name: line.strip_prefix("$ cd ").unwrap(),
        })
    } else if line == "$ ls" {
        ParsedLine::ListDirs
    } else if line.starts_with("dir ") {
        ParsedLine::Directory {
            dir_name: line.strip_prefix("dir ").unwrap(),
        }
    } else {
        let mut parts = line.split(' ');
        let file_size_part = parts.next().unwrap_or_else(|| {
            panic!("expected to find file size separated from file name by space character")
        });
        let file_size = str::parse(file_size_part)
            .unwrap_or_else(|_| panic!("failed to parse file size as number"));
        ParsedLine::File { file_size }
    }
}

fn main() {
    let root = DirInfo {
        size: 0,
        entries: HashMap::new(),
        parent: None,
    };
    let mut current_dir = Rc::new(RefCell::new(&root));

    for line in utils::read_input().lines().map(parse_line) {
        match line {
            ParsedLine::ChangeDir(ChangeDirDest::Up) => {
                let new_dir = current_dir.borrow().parent.as_ref().unwrap().clone();
                current_dir = new_dir;
            }
            ParsedLine::ChangeDir(ChangeDirDest::Root) => {
                current_dir = Rc::new(RefCell::new(&root))
            }
            ParsedLine::ChangeDir(ChangeDirDest::ChildDir { dir_name }) => {
                let child_dir_entry = current_dir.borrow().entries.get(dir_name);
                if let Some(DirEntry::dir(child_dir_info)) = child_dir_entry {
                    current_dir = Rc::new(RefCell::new(child_dir_info));
                } else {
                    panic!("failed to find child dir to cd into");
                }
            }
            ParsedLine::Directory { dir_name } => {
                current_dir.borrow_mut().entries.insert(
                    dir_name.to_owned(),
                    DirEntry::new_child_dir(current_dir.clone()),
                );
                // entries
                //     .entry(dir_name.to_owned())
                //     .or_insert_with(|| );
            }
            // ParsedLine::File { file_size } => {
            //     todo!()
            // }
            // ParsedLine::ListDirs => {
            //     // Don't actually need to do anything here.
            // }
            ,
            _ => {
                todo!()
            }
        }
    }
}
