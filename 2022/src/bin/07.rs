use std::{cell::RefCell, rc::Rc};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/07")));
    println!("02: {}", part02(include_str!("../inputs/07")));
}

#[derive(Debug, PartialEq)]
struct Dir {
    name: String,
    sub_dirs: RefCell<Vec<Rc<Dir>>>,
    files: RefCell<Vec<Rc<File>>>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sub_dirs: RefCell::new(Vec::new()),
            files: RefCell::new(Vec::new()),
        }
    }

    fn get_dir(&self, name: &str) -> Rc<Dir> {
        self.sub_dirs
            .borrow()
            .iter()
            .find(|&sub_dir| sub_dir.name == name.to_string())
            .unwrap()
            .to_owned()
    }

    fn get_size(&self) -> usize {
        let mut size = 0;
        for dir in self.sub_dirs.borrow().iter() {
            size += dir.get_size();
        }
        for file in self.files.borrow().iter() {
            size += file.size;
        }
        size
    }
}

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    root: RefCell<Rc<Dir>>,
    dirs: RefCell<Vec<Rc<Dir>>>,
    cwd: RefCell<String>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: RefCell::new(Rc::new(Dir::new("/"))),
            dirs: RefCell::new(Vec::new()),
            cwd: RefCell::new("/".to_string()),
        }
    }

    fn add_dir(&self, name: &str) {
        let dir = self.get_dir(&self.cwd.borrow());
        let new_dir = Rc::new(Dir::new(name));
        dir.sub_dirs.borrow_mut().push(new_dir.clone());
        self.dirs.borrow_mut().push(new_dir);
    }

    fn get_dir(&self, path: &str) -> Rc<Dir> {
        let path = if path.starts_with('/') {
            let (_, path) = path.split_at(1);
            path.to_string()
        } else {
            let cwd = self.cwd.borrow();
            let path = if cwd.eq("/") {
                format!("{}{}", cwd, path)
            } else {
                format!("{}/{}", cwd, path)
            };
            let (_, path) = path.split_at(1);
            path.to_string()
        };

        if path.is_empty() {
            self.root.borrow().to_owned()
        } else {
            let mut dir: Rc<Dir> = self.root.borrow().to_owned();
            for comp in path.split('/') {
                dir = dir.get_dir(comp);
            }
            dir
        }
    }

    fn change_dir(&self, path: &str) {
        let mut cwd = self.cwd.borrow_mut();
        if path.starts_with('/') {
            *cwd = path.to_string();
        } else if path == ".." {
            let mut comps = cwd.split('/');
            comps.next_back();
            *cwd = comps.collect::<Vec<_>>().join("/");
            if cwd.is_empty() {
                *cwd = "/".to_string();
            }
        } else {
            *cwd = if cwd.eq("/") {
                format!("{}{}", cwd, path)
            } else {
                format!("{}/{}", cwd, path)
            };
        }
    }

    fn add_file(&self, name: &str, size: usize) {
        let dir = self.get_dir(&self.cwd.borrow());
        dir.files.borrow_mut().push(Rc::new(File::new(name, size)));
    }
}

fn part01(input: &str) -> usize {
    let mut lines = input.lines();
    assert_eq!(lines.next(), Some("$ cd /"));
    let fs = FileSystem::new();
    for line in lines {
        if line == "$ ls" {
            continue;
        }

        if !line.starts_with('$') {
            let (a, b) = line.split_once(' ').unwrap();
            if a == "dir" {
                fs.add_dir(b);
                continue;
            }
            if let Ok(size) = a.parse::<usize>() {
                fs.add_file(b, size);
                continue;
            }
            unreachable!()
        }

        if line.starts_with("$ cd") {
            let (_, s) = line.split_at(5);
            fs.change_dir(s);
        }
    }
    let mut sum = 0;
    for dir in fs.dirs.borrow().iter() {
        let size = dir.get_size();
        if size <= 100000 {
            sum += size;
        }
    }
    sum
}

fn part02(input: &str) -> usize {
    let mut lines = input.lines();
    assert_eq!(lines.next(), Some("$ cd /"));
    let fs = FileSystem::new();
    for line in lines {
        if line == "$ ls" {
            continue;
        }

        if !line.starts_with('$') {
            let (a, b) = line.split_once(' ').unwrap();
            if a == "dir" {
                fs.add_dir(b);
                continue;
            }
            if let Ok(size) = a.parse::<usize>() {
                fs.add_file(b, size);
                continue;
            }
            unreachable!()
        }

        if line.starts_with("$ cd") {
            let (_, s) = line.split_at(5);
            fs.change_dir(s);
        }
    }

    let available = 70_000_000;
    let target_free = 30_000_000;
    let used = fs.root.borrow().get_size();
    let free = available - used;
    let needed = target_free - free;

    let mut candidates = Vec::new();
    for dir in fs.dirs.borrow().iter() {
        let size = dir.get_size();
        if size >= needed {
            candidates.push(size);
        }
    }
    candidates.sort();
    *candidates.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs() {
        let fs = FileSystem::new();
        assert_eq!(fs.get_dir("/"), fs.root.borrow().to_owned());
        fs.add_dir("a");
        assert_eq!(
            fs.get_dir("/a"),
            fs.root.borrow().sub_dirs.borrow()[0].to_owned()
        );
        fs.change_dir("a");
        fs.add_dir("b");
        assert_eq!(
            fs.get_dir("/a/b"),
            fs.root.borrow().sub_dirs.borrow()[0].sub_dirs.borrow()[0].to_owned()
        );
        fs.change_dir("..");
        fs.add_dir("c");
        assert_eq!(
            fs.get_dir("c"),
            fs.root.borrow().sub_dirs.borrow()[1].to_owned()
        );
    }

    #[test]
    fn test_files() {
        let fs = FileSystem::new();
        fs.add_file("f1.txt", 42);
    }

    #[test]
    fn example01() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        assert_eq!(part01(input), 95437);
    }

    #[test]
    fn example02() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        assert_eq!(part02(input), 24933642);
    }
}
