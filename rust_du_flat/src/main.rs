use std::path::{Path, PathBuf};
use std::{fs};

fn main() {
    println!("Hello, world!");

    // This implementation uses a flat storage.

    let path = Path::new("/home");
    let mut root_dir = Directory::new(path.to_path_buf(), None);
    root_dir.scan();
    // Print directory structure
    root_dir.print_tree(0);
}



/// Data for a directory.
struct Directory {
    path: str,
    size: u64
}


fn scan_directories{

}


impl Directory {
    fn scan(self: &mut Directory){
        let mut size_accumulator = 0;

        match fs::read_dir(Path::new(&self.path)) {
            Ok(cildren) => {
                for entry in cildren {
                    // Result or panic.
                    let path = entry.unwrap().path();

                    if path.is_dir() {
                        let mut child_dir = Directory::new(path, Some(Rc::downgrade(self)));

                        // Recursively scan the child directory
                        child_dir.scan();

                        // Add child to this directory
                        self.children.borrow_mut().push(child_dir);
                    }
                    else if path.is_file(){
                        // add the size of the file to the sum
                        // May not have permission to get the metadata.
                        match fs::metadata(&path) {
                            Ok(metadata) => {
                                size_accumulator += metadata.len();
                            }
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        }
                    }
                }
            }

            Err(e) => {
                println!("{:?}", e);
            }

        }
        // Update the size of this directory
        *self.size.borrow_mut() = size_accumulator;
        *self.last_scan.borrow_mut() = Some(std::time::SystemTime::now());
    }
    /// Print the directory tree for debugging
    fn print_tree(&self, level: usize) {
        let indent = "  ".repeat(level);
        println!(
            "{}{} - {} bytes",
            indent,
            self.path.display(),
            *self.size.borrow()
        );

        for child in self.children.borrow().iter() {
            child.print_tree(level + 1);
        }
    }
}


