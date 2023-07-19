// Re-implementation of day07 without the use of Rc<RefCell>>. This is done by having a central
// struct that owns the Folders and files, and each filesystem item being a handle to one of the
// indices of that central struct. This should remove the overhead added by the runtime borrow
// checking done by RefCells.

use std::collections::VecDeque;

type FolderHandle = usize;
struct FileSystem {
    folders: Vec<FsFolder>,
}

impl FileSystem {
    fn new() -> Self {
        let root_folder = FsFolder {
            name: String::from("/"),
            files_size: 0,
            subfolders: Vec::new(),
            parent: 0,
        };

        Self {
            folders: vec![root_folder],
        }
    }

    fn add_file_to_folder(&mut self, folder_handle: FolderHandle, file_size: usize) {
        self.folders
            .get_mut(folder_handle)
            .expect("Invalid folder handle.")
            .add_size(file_size);
    }

    fn create_subfolder(&mut self, parent_handle: FolderHandle, child_name: &str) -> FolderHandle {
        let child_folder = FsFolder::with_parent(child_name, parent_handle);
        self.folders.push(child_folder);
        let child_handle = self.folders.len() - 1;
        self.folders
            .get_mut(parent_handle)
            .expect("Invalid folder handle.")
            .add_subfolder(child_handle);
        child_handle
    }

    fn get_all_subfolders(&self, parent_handle: FolderHandle) -> &[FolderHandle] {
        &self
            .folders
            .get(parent_handle)
            .expect("Invalid folder handle")
            .subfolders
    }
    fn get_subfolder_handle(&self, parent_handle: FolderHandle, child_name: &str) -> FolderHandle {
        let parent_folder = self
            .folders
            .get(parent_handle)
            .expect("Invalid folder handle.");

        *parent_folder
            .subfolders
            .iter()
            .find(|&&c_handle| {
                self.folders
                    .get(c_handle)
                    .expect("Invalid folder handle.")
                    .name
                    == child_name
            })
            .expect("Subfolder does not exist")
    }
    fn get_parent_folder(&self, folder_handle: FolderHandle) -> FolderHandle {
        self.folders
            .get(folder_handle)
            .expect("Invalid folder handle.")
            .parent
    }

    fn get_folder_recursive_size(&self, folder_handle: FolderHandle) -> usize {
        let folder = self
            .folders
            .get(folder_handle)
            .expect("Invalid folder handle.");
        let mut total = folder.files_size;

        for child_handle in folder.subfolders.iter() {
            total += self.get_folder_recursive_size(*child_handle);
        }
        total
    }
}

struct FsFolder {
    name: String,
    files_size: usize, // Size of files contained in folder
    subfolders: Vec<FolderHandle>,
    parent: FolderHandle,
}

impl FsFolder {
    fn with_parent(name: &str, parent: FolderHandle) -> Self {
        FsFolder {
            name: name.to_owned(),
            files_size: 0,
            subfolders: Vec::new(),
            parent,
        }
    }

    fn add_subfolder(&mut self, subfolder: FolderHandle) {
        self.subfolders.push(subfolder);
    }

    fn add_size(&mut self, new_file_size: usize) {
        self.files_size += new_file_size
    }
}

pub fn part1(lines: Vec<String>) -> usize {
    let filesystem = generate_file_structure(&lines);
    let mut folders: VecDeque<FolderHandle> = VecDeque::new();
    folders.push_front(0); // Root folder handle = 0

    let mut total = 0;
    while let Some(handle) = folders.pop_back() {
        let size = filesystem.get_folder_recursive_size(handle);
        if size <= 100000 {
            total += size;
        }

        for child_handle in filesystem.get_all_subfolders(handle) {
            folders.push_front(*child_handle);
        }
    }
    total
}

pub fn part2(lines: Vec<String>) -> usize {
    let filesystem = generate_file_structure(&lines);
    let mut sizes: Vec<usize> = Vec::new();

    let mut folder_queue: VecDeque<FolderHandle> = VecDeque::new();
    // Get full FS size by getting recursive size of root dir (handle 0)
    let used_size: usize = filesystem.get_folder_recursive_size(0);

    folder_queue.push_front(0); // Push root dir

    while let Some(folder_handle) = folder_queue.pop_back() {
        sizes.push(filesystem.get_folder_recursive_size(folder_handle));

        for child_handle in filesystem.get_all_subfolders(folder_handle) {
            folder_queue.push_front(*child_handle);
        }
    }
    sizes.sort();

    const TOTAL_SIZE: usize = 70000000;
    const NEEDED_SIZE: usize = 30000000;
    let free_size: usize = TOTAL_SIZE - used_size;
    let missing_size: usize = NEEDED_SIZE - free_size;

    sizes.into_iter().find(|&n| n >= missing_size).unwrap()
}

fn generate_file_structure(lines: &[String]) -> FileSystem {
    let mut i = 1; // First line is `cd /`, can skip
    let n = lines.len();

    let mut filesystem = FileSystem::new();
    let mut cwd: FolderHandle = 0; // Root directory should always have a zero handle

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
                    filesystem.create_subfolder(cwd, name);
                } else {
                    // File (format: size name)
                    let size: usize = line.split(' ').next().unwrap().parse().unwrap();
                    filesystem.add_file_to_folder(cwd, size);
                }
                i += 1;
            }
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split_at(5).1;
            if dir_name == ".." {
                cwd = filesystem.get_parent_folder(cwd);
            } else {
                cwd = filesystem.get_subfolder_handle(cwd, dir_name);
            }
            i += 1;
        }
    }
    filesystem
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
