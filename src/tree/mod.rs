use std::fs;
use std::io::Error;
use std::path::PathBuf;
mod item;
mod metadata;
pub use self::item::{path_name, TreeItem};
use self::metadata::FileType;

/// returns the "length" of a path.
fn path_len(p: &PathBuf) -> usize {
    p.components().collect::<Vec<_>>().len()
}

pub struct TreeMaker {
    max_depth: isize,
    root_len: usize,
    show_hidden: bool,
}

impl TreeMaker {
    pub fn new(max_depth: isize, root_dir: &str, show_hidden: bool) -> TreeMaker {
        TreeMaker {
            max_depth,
            show_hidden,
            root_len: path_len(&PathBuf::from(root_dir)),
        }
    }
    pub fn make(&self, dir: &str) -> Vec<Result<TreeItem, Error>> {
        let root_len = self.root_len;
        let max_depth = self.max_depth;
        let mut stack = vec![];
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.map(|r| r.and_then(|e| metadata::info(e))) {
                match entry {
                    Ok((path, ftype)) => {
                        if !self.show_hidden && path_name(&path).starts_with(".") {
                            continue;
                        }

                        match ftype {
                            FileType::File => stack.push(Ok(TreeItem::from(path))),
                            FileType::Directory => {
                                let cur_depth = path_len(&path) - root_len - 1;

                                let should_pass = if max_depth < 0 {
                                    true
                                } else {
                                    cur_depth < self.max_depth as usize
                                };

                                if should_pass {
                                    for item in self.make(path.to_str().unwrap()) {
                                        stack.push(item)
                                    }
                                }
                            }
                            FileType::Symlink => (), // TODO: add support for symlinks
                        }
                    }
                    Err(e) => stack.push(Err(e)),
                }
            }
        }

        stack
    }
}
