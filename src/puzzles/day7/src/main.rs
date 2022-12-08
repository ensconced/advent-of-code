use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct DirInfo<'a> {
    size: u32,
    entries: RefCell<HashMap<String, DirEntry<'a>>>,
    parent: Option<Rc<DirInfo<'a>>>,
}

enum DirEntry<'a> {
    Dir(Rc<DirInfo<'a>>),
}

impl<'a> DirEntry<'a> {
    fn new_child_dir(parent: &Rc<DirInfo<'a>>) -> Self {
        Self::Dir(Rc::new(DirInfo {
            size: 0,
            entries: RefCell::new(HashMap::new()),
            parent: Some(parent.clone()),
        }))
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

fn find_child_dir<'a>(parent_dir: Rc<DirInfo<'a>>, child_name: &str) -> Rc<DirInfo<'a>> {
    let current_dir_contents = parent_dir.entries.borrow();
    if let Some(DirEntry::Dir(child_dir_info)) = current_dir_contents.get(child_name) {
        child_dir_info.clone()
    } else {
        panic!("failed to find child dir to cd into");
    }
}

fn main() {
    let root = Rc::new(DirInfo {
        size: 0,
        entries: RefCell::new(HashMap::new()),
        parent: None,
    });
    let mut current_dir = root.clone();

    for line in utils::read_input().lines().map(parse_line) {
        match line {
            ParsedLine::ChangeDir(ChangeDirDest::Up) => {
                let new_dir = current_dir.parent.as_ref().unwrap().clone();
                current_dir = new_dir.clone();
            }
            ParsedLine::ChangeDir(ChangeDirDest::Root) => {
                current_dir = root.clone();
            }
            ParsedLine::ChangeDir(ChangeDirDest::ChildDir { dir_name }) => {
                current_dir = find_child_dir(current_dir, dir_name);
            }
            ParsedLine::Directory { dir_name } => {
                current_dir
                    .entries
                    .borrow_mut()
                    .insert(dir_name.to_owned(), DirEntry::new_child_dir(&current_dir));
            }
            // ParsedLine::File { file_size } => {
            //     current_dir
            //         .entries
            //         .borrow_mut()
            //         .insert(dir_name.to_owned(), DirEntry::new_child_dir(&current_dir));
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
