use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct DirContent {
    name: String,
    type_info: FileType,
    parent: Weak<RefCell<DirContent>>,
}

impl DirContent {
    pub fn file(name: impl AsRef<str>, size: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.as_ref().to_owned(),
            type_info: FileType::File(size),
            parent: Weak::new(),
        }))
    }

    pub fn dir(name: impl AsRef<str>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.as_ref().to_owned(),
            type_info: FileType::Dir(vec![]),
            parent: Weak::new(),
        }))
    }
}

#[derive(Debug)]
pub enum FileType {
    File(usize),
    Dir(Vec<Rc<RefCell<DirContent>>>),
}
impl FileType {
    pub fn is_file(&self) -> bool {
        match self {
            FileType::File(_) => true,
            _ => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            FileType::Dir(_) => true,
            _ => false,
        }
    }
}

impl DirContent {
    pub fn size(&self) -> usize {
        match &self.type_info {
            FileType::File(size) => *size,
            FileType::Dir(contents) => contents.iter().map(|content| content.borrow().size()).sum(),
        }
    }

    pub fn path(&self) -> String {
        let Some(parent) = self.parent.upgrade() else {
            return self.name.clone();
        };

        let parent = parent.borrow();
        parent.path() + self.name.as_ref()
    }

    pub fn type_info(&self) -> &FileType {
        &self.type_info
    }

    pub fn insert(s: Rc<RefCell<Self>>, other: Rc<RefCell<DirContent>>) -> Result<(), String> {
        match &mut s.borrow_mut().type_info {
            FileType::File(_) => Err(format!("attempt to insert into file {}", s.borrow().path())),
            FileType::Dir(contents) => {
                other.borrow_mut().parent = Rc::downgrade(&s);
                contents.push(other);
                Ok(())
            }
        }
    }

    pub fn navigate(&self, name: impl AsRef<str>) -> Option<Rc<RefCell<Self>>> {
        match self.type_info {
            FileType::File(_) => None,
            FileType::Dir(ref contents) => contents
                .iter()
                .find(|entry| entry.borrow().name == name.as_ref())
                .map(Rc::clone),
        }
    }

    pub fn files(s: &Rc<RefCell<Self>>) -> impl Iterator<Item = Rc<RefCell<DirContent>>> {
        use FileType::*;
        let mut v = vec![];
        v.push(Rc::clone(s));
        match s.borrow().type_info {
            File(_) => {}
            Dir(ref contents) => {
                for file in contents {
                    v.extend(DirContent::files(file))
                }
            }
        }

        v.into_iter()
    }
}

fn process_input(input: &str) -> Rc<RefCell<DirContent>> {
    let root = DirContent::dir("/");
    let mut current_dir = Rc::clone(&root);
    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "dir" => {
                let dir_name = split.next().unwrap();
                DirContent::insert(Rc::clone(&current_dir), DirContent::dir(dir_name))
                    .expect("tried inserting a file into a non-directory");
            }
            "$" => match split.next().unwrap() {
                "cd" => {
                    let next_dir_name = split.next().unwrap();
                    match next_dir_name {
                        "/" => {
                            current_dir = Rc::clone(&root);
                            continue;
                        }
                        ".." => {
                            let parent = current_dir
                                .borrow()
                                .parent
                                .upgrade()
                                .unwrap_or_else(|| Rc::clone(&root));
                            current_dir = parent;
                        }
                        _ => {
                            let next_dir = current_dir.borrow().navigate(next_dir_name).expect(&format!("attempted to navigate to file/non-existant directory {next_dir_name}"));
                            current_dir = next_dir;
                        }
                    }
                }
                _ => {}
            },
            token if token.parse::<usize>().is_ok() => {
                let size = token.parse::<usize>().unwrap();
                let name = split.next().unwrap();
                DirContent::insert(Rc::clone(&current_dir), DirContent::file(name, size))
                    .expect("tried inserting a file into a non-directory");
            }
            _ => {}
        }
    }
    root
}

pub fn process_part1(input: &str) -> usize {
    let root = process_input(input);
    DirContent::files(&root)
        .filter(|file| file.borrow().type_info().is_dir())
        .map(|file| file.borrow().size())
        .filter(|&size| size <= 100000)
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let fs_space = 70_000_000;
    let required_space = 30_000_000;
    let root_rc = process_input(input);
    let root = root_rc.borrow();

    let unused_space = fs_space - root.size();
    let space_that_must_be_freed = required_space - unused_space;

    DirContent::files(&root_rc)
        .filter(|file| file.borrow().type_info().is_dir())
        .map(|file| file.borrow().size())
        .filter(|&size| size >= space_that_must_be_freed)
        .min()
        .unwrap_or(0)
}
