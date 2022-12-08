use std::vec;

struct Dir {
    // name: String,
    subdirs: Vec<Dir>,
    files: Vec<File>,
}

impl Dir {
    fn size(&self) -> u32 {
        return self.files.iter().fold(0, |acc, x| acc + x.size)
            + self.subdirs.iter().fold(0, |acc, x| acc + x.size());
    }

    fn dirs_of_at_most_size(&self, size: u32) -> Vec<&Dir> {
        let mut result = vec![];
        if self.size() <= size {
            result.push(self);
        }
        for dir in self.subdirs.iter() {
            result.append(&mut dir.dirs_of_at_most_size(size));
        }
        result
    }

    fn dirs_of_at_least_size(&self, size: u32) -> Vec<&Dir> {
        let mut result = vec![];
        if self.size() >= size {
            result.push(self);
        }
        for dir in self.subdirs.iter() {
            result.append(&mut dir.dirs_of_at_least_size(size));
        }
        result
    }
}

struct File {
    // name: String,
    size: u32,
}

pub fn run(input: String) -> (String, String) {
    let root = build_filesystem_tree(input);

    (
        root.dirs_of_at_most_size(100000)
            .iter()
            .fold(0, |acc, x| acc + x.size())
            .to_string(),
        root.dirs_of_at_least_size(30000000 - (70000000 - root.size()))
            .iter()
            .min_by(|x, y| x.size().cmp(&y.size()))
            .unwrap()
            .size()
            .to_string(),
    )
}

fn build_filesystem_tree(input: String) -> Dir {
    let mut curdir = Dir {
        // name: "/".to_owned(),
        subdirs: vec![],
        files: vec![],
    };
    let mut dirstack: Vec<Dir> = vec![];
    for line in input.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            (curdir, dirstack) = handle_cd(&parts, dirstack, curdir);
        } else if parts[0] != "$" {
            if let Some(file) = handle_ls(parts) {
                curdir.files.push(file);
            }
        }
    }
    while !dirstack.is_empty() {
        let mut parent = dirstack.pop().unwrap();
        parent.subdirs.push(curdir);
        curdir = parent;
    }
    curdir
}

fn handle_ls(parts: Vec<&str>) -> Option<File> {
    let size_or_dir = parts[0].parse::<u32>();
    match size_or_dir {
        Ok(size) => Some(File {
            // name: parts[1].to_owned(),
            size,
        }),
        _ => None,
    }
}

fn handle_cd(parts: &[&str], mut dirstack: Vec<Dir>, curdir: Dir) -> (Dir, Vec<Dir>) {
    let dirname = parts[2];
    if dirname == ".." {
        let mut parent = dirstack.pop().unwrap();
        parent.subdirs.push(curdir);
        (parent, dirstack)
    } else {
        let next_dir = Dir {
            // name: dirname.to_owned(),
            subdirs: vec![],
            files: vec![],
        };
        dirstack.push(curdir);
        (next_dir, dirstack)
    }
}
