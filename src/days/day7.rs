use std::{
    fs::File,
    io::BufRead,
    io::BufReader,
    path::{Path, PathBuf}, env::current_dir,
};

#[derive(Debug)]
enum DirElem {
    FileNode {
        name: String,
        size: isize,
    },
    DirNode {
        name: String,
        contents: Vec<DirElem>,
    },
}

impl DirElem {
    pub fn get_name(&self) -> &str {
        match &self {
            DirElem::DirNode { name, .. } => name,
            DirElem::FileNode { name, .. } => name,
        }
    }
}

#[derive(Debug)]
struct DirTree {
    root: DirElem,
}

impl DirTree {
    fn new() -> Self {
        Self {
            root: DirElem::DirNode {
                name: "/".to_string(),
                contents: Vec::new(),
            },
        }
    }

    fn check_name(test_name: &str, elems: &Vec<DirElem>) -> bool {
        elems.iter().all(|node| node.get_name() != test_name)
    }

    fn add_dir_elem(new_elem: DirElem, contents: &mut Vec<DirElem>) -> Result<(), &'static str> {
        if !DirTree::check_name(new_elem.get_name(), contents) {
            return Err("Error adding DirElem - already exists");
        }
        contents.push(new_elem);
        Ok(())
    }

    fn get_dir_from_path<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut DirElem> {
        path.as_ref()
            .iter()
            .try_fold(&mut self.root, |curr_dir, next_dir| match curr_dir {
                _ if curr_dir.get_name() == next_dir.to_str().unwrap() => {
                    Some(curr_dir)},
                DirElem::DirNode { contents, .. } => {
                    contents.into_iter().find(|x| x.get_name() == next_dir)
                }
                _ => None,
            })
    }

    fn add_dir_elem_path<P: AsRef<Path>>(
        &mut self,
        new_elem: DirElem,
        path: P,
    ) -> Result<(), &'static str> {
        if let Some(DirElem::DirNode { contents, .. }) = self.get_dir_from_path(path) {
            return DirTree::add_dir_elem(new_elem, contents);
        }

        Err("Error adding element to path")
    }
}

fn parse_dirs<B>(input_reader: B) -> DirTree
where
    B: BufRead,
{
    input_reader
        .lines()
        .fold(
            (DirTree::new(), PathBuf::from("/")),
            |(mut tree, path), line| {
                let parsed_line = line.unwrap_or_default();
                match parsed_line.trim_start_matches('$').trim().split_once(' ') {
                    Some(("cd", "/")) => (tree, PathBuf::from("/")),
                    Some(("cd", "..")) => {
                        (tree, path.parent().unwrap_or(Path::new("/")).to_owned())
                    }
                    Some(("cd", dir_name)) => (tree, path.join(dir_name).to_owned()),
                    Some(("dir", dir_name)) => {
                        tree.add_dir_elem_path(
                            DirElem::DirNode {
                                name: dir_name.to_owned(),
                                contents: Vec::new(),
                            },
                            &path,
                        )
                        .ok();
                        (tree, path)
                    }
                    Some((file_size, file_name)) => {
                        tree.add_dir_elem_path(
                            DirElem::FileNode {
                                name: file_name.to_owned(),
                                size: file_size.parse().unwrap(),
                            },
                            &path,
                        )
                        .ok();
                        (tree, path)},
                    _ => (tree, path),
                }
            },
        )
        .0
}

pub fn print_answer() {
    let mut buf_read = BufReader::new(File::open("data/input_day7").unwrap());
    let file_tree = parse_dirs(buf_read);
    //println!("Parsed tree: {:?}", file_tree);
}
