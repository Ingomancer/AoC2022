use std::vec;

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Dir>,
    files: Vec<File>,
}

impl Dir {
    fn size(&self) -> u32 {
        return self.files.iter().fold(0, |acc, x| acc + x.size)
            + self.subdirs.iter().fold(0, |acc, x| acc + x.size());
    }

    fn find_dirs_below_size(&self, size: u32) -> Vec<&Dir> {
        let mut result = vec![];
        if self.size() <= size {
            result.push(self);
        }
        for dir in self.subdirs.iter() {
            result.append(&mut dir.find_dirs_below_size(size));
        }
        result
    }

    fn find_dirs_above_size(&self, size: u32) -> Vec<&Dir> {
        let mut result = vec![];
        if self.size() >= size {
            result.push(self);
        }
        for dir in self.subdirs.iter() {
            result.append(&mut dir.find_dirs_above_size(size));
        }
        result
    }
}

#[derive(Clone, Debug)]
struct File {
    size: u32,
}

pub fn run(input: String) {
    let mut curdir = Dir {
        name: "/".to_owned(),
        subdirs: vec![],
        files: vec![],
    };
    let mut dirstack: Vec<Dir> = vec![];
    for line in input.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let dirname = parts[2];
            if dirname == ".." {
                let mut parent = dirstack.pop().unwrap();
                parent.subdirs.push(curdir);
                curdir = parent;
            } else {
                let position = curdir
                    .subdirs
                    .iter()
                    .position(|dir| dir.name == dirname)
                    .unwrap();
                let next_dir = curdir.subdirs.remove(position);
                dirstack.push(curdir.clone());
                curdir = next_dir;
            }
        } else if parts[0] != "$" {
            let size_or_dir = parts[0].parse::<u32>();
            match size_or_dir {
                Ok(size) => curdir.files.push(File { size }),
                Err(_) => curdir.subdirs.push(Dir {
                    name: parts[1].to_owned(),
                    subdirs: vec![],
                    files: vec![],
                }),
            }
        }
    }
    while !dirstack.is_empty() {
        let mut parent = dirstack.pop().unwrap();
        parent.subdirs.push(curdir);
        curdir = parent;
    }

    println!(
        "{:?}",
        curdir
            .find_dirs_below_size(100000)
            .iter()
            .fold(0, |acc, x| acc + x.size())
    );
    let candidates = curdir.find_dirs_above_size(30000000 - (70000000 - curdir.size()));
    println!(
        "{}",
        candidates
            .iter()
            .min_by(|x, y| x.size().cmp(&y.size()))
            .unwrap()
            .size()
    );
}
