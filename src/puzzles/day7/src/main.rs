use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    str::Lines,
};

struct DirInfo<'a> {
    size: Cell<u32>,
    entries: RefCell<HashMap<String, Rc<DirInfo<'a>>>>,
    parent: Option<Rc<DirInfo<'a>>>,
}

impl<'a> DirInfo<'a> {
    fn add_file(&self, file_size: u32) {
        self.size.set(self.size.get() + file_size);
        if let Some(parent) = &self.parent {
            parent.add_file(file_size);
        }
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
    if let Some(child_dir_info) = current_dir_contents.get(child_name) {
        child_dir_info.clone()
    } else {
        panic!("failed to find child dir to cd into");
    }
}

struct FilesystemSummary<'a> {
    root: Rc<DirInfo<'a>>,
    all_dirs: Vec<Rc<DirInfo<'a>>>,
}

fn build_filesystem(lines: Lines) -> FilesystemSummary {
    let mut all_dirs = vec![];

    let mut create_child_dir = |parent| {
        let result = Rc::new(DirInfo {
            size: Cell::new(0),
            entries: RefCell::new(HashMap::new()),
            parent,
        });
        all_dirs.push(result.clone());
        result
    };

    let root = create_child_dir(None);
    let mut current_dir = root.clone();

    for line in lines.map(parse_line) {
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
                let new_dir = create_child_dir(Some(current_dir.clone()));

                current_dir
                    .entries
                    .borrow_mut()
                    .insert(dir_name.to_owned(), new_dir);
            }
            ParsedLine::File { file_size } => {
                current_dir.add_file(file_size);
            }
            ParsedLine::ListDirs => {
                // Don't actually need to do anything here.
            }
        }
    }
    FilesystemSummary { root, all_dirs }
}

fn main() {
    let input = utils::read_input();
    let FilesystemSummary { root, all_dirs } = build_filesystem(input.lines());
    let part_1_answer = all_dirs
        .iter()
        .map(|dir| dir.size.get())
        .filter(|size| *size <= 100000)
        .sum::<u32>();

    let total_disk_space = 70000000;
    let required_free_space = 30000000;
    let current_free_space = total_disk_space - root.size.get();
    let further_space_to_delete = required_free_space - current_free_space;

    let part_2_answer = all_dirs
        .iter()
        .map(|dir| dir.size.get())
        .filter(|size| *size >= further_space_to_delete)
        .min()
        .unwrap();

    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
