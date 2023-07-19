use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

enum FsItem {
    Folder(FsFolder),
    File(FsFile),
}

struct FsFolder {
    name: String,
    parent: Option<Weak<RefCell<FsFolder>>>,
    subfolders: Vec<Rc<RefCell<FsFolder>>>,
    files: Vec<FsFile>,
}

impl FsFolder {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            parent: None,
            subfolders: Vec::new(),
            files: Vec::new(),
        }
    }

    fn with_parent(name: &str, parent: Weak<RefCell<FsFolder>>) -> Self {
        Self {
            name: name.to_owned(),
            parent: Some(parent),
            subfolders: Vec::new(),
            files: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.subfolders
            .iter()
            .map(|folder| folder.borrow().size())
            .sum::<usize>()
            + self.files.iter().map(|f| f.size).sum::<usize>()
    }

    fn add_child(&mut self, child: FsItem) {
        match child {
            FsItem::File(f) => self.files.push(f),
            FsItem::Folder(f) => self.subfolders.push(Rc::new(RefCell::new(f))),
        }
    }

    fn get_subfolder(&self, name: &str) -> Rc<RefCell<FsFolder>> {
        self.subfolders
            .iter()
            .find(|&f| f.borrow().name == name)
            .expect("Subfolder does not exist")
            .clone()
    }

    fn get_all_subfolders(&self) -> &Vec<Rc<RefCell<FsFolder>>> {
        &self.subfolders
    }
}

struct FsFile {
    size: usize,
}

impl FsFile {
    fn from_size(size: usize) -> Self {
        Self { size }
    }
}

pub fn part1(lines: Vec<String>) -> usize {
    let root_dir = generate_file_structure(&lines);

    let mut folder_queue: VecDeque<Rc<RefCell<FsFolder>>> = VecDeque::new();
    folder_queue.push_front(root_dir);

    let mut total = 0;
    while let Some(folder) = folder_queue.pop_back() {
        let folder = &*folder.borrow();
        let size = folder.size();
        if size <= 100000 {
            total += size;
        }

        for sf in folder.get_all_subfolders() {
            folder_queue.push_front(sf.clone());
        }
    }
    total
}

pub fn part2(lines: Vec<String>) -> usize {
    let root_dir = generate_file_structure(&lines);
    let mut sizes: Vec<usize> = Vec::new();

    let mut folder_queue: VecDeque<Rc<RefCell<FsFolder>>> = VecDeque::new();
    let used_size: usize = root_dir.borrow().size();
    folder_queue.push_front(root_dir);

    while let Some(folder) = folder_queue.pop_back() {
        let folder = folder.borrow();
        sizes.push(folder.size());

        for sf in folder.get_all_subfolders() {
            folder_queue.push_front(sf.clone());
        }
    }
    sizes.sort();

    const TOTAL_SIZE: usize = 70000000;
    const NEEDED_SIZE: usize = 30000000;
    let free_size: usize = TOTAL_SIZE - used_size;
    let missing_size: usize = NEEDED_SIZE - free_size;

    sizes.into_iter().find(|&n| n >= missing_size).unwrap()
}

fn generate_file_structure(lines: &[String]) -> Rc<RefCell<FsFolder>> {
    // RefCells get ugly really quickly, probably don't need to use them as extensivly if you
    // refactor code to not use structs and just use a HashMap with (path, size), but was still
    // informative to learn how to use the Rc<RefCell<_>> pattern
    let mut i = 1; // First line is `cd /`, can skip
    let n = lines.len();

    let mut cwd = Rc::new(RefCell::new(FsFolder::new("/")));
    let root_dir = cwd.clone();

    while i < n {
        let line = &lines[i];

        if line == "$ ls" {
            // Read lines until they start with $ (next command)
            i += 1;
            while i < n {
                let line = &lines[i];

                if line.starts_with('$') {
                    // New command, not data
                    break;
                }

                if line.starts_with("dir ") {
                    // Directory
                    let name = line.split(' ').nth(1).unwrap();
                    let new_folder = FsFolder::with_parent(name, Rc::downgrade(&cwd));
                    cwd.borrow_mut().add_child(FsItem::Folder(new_folder));
                } else {
                    // File (format: size name)
                    let size: usize = line.split(' ').next().unwrap().parse().unwrap();
                    cwd.borrow_mut()
                        .add_child(FsItem::File(FsFile::from_size(size)));
                }
                i += 1;
            }
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split_at(5).1;
            if dir_name == ".." {
                let parent_dir = cwd
                    .borrow()
                    .parent
                    .clone()
                    .expect("No parent directory.")
                    .upgrade()
                    .unwrap();
                cwd = parent_dir;
            } else {
                cwd = cwd.clone().borrow().get_subfolder(dir_name);
            }
            i += 1;
        }
    }
    root_dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part1(data), 95437);
    }

    #[test]
    fn test_part2() {
        let data = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part2(data), 24933642);
    }
}
