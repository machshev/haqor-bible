// Load text from bible verse (SQLite)

use dirs;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Bible {
    pub name: String,
    file_path: PathBuf,
}

#[derive(Debug)]
pub struct Library {
    base_path: PathBuf,
}

impl Default for Library {
    fn default() -> Self {
        let base_path: PathBuf = match env::var("HAQOR_LIBRARY_PATH") {
            Ok(i) => PathBuf::from(i),
            _ => dirs::data_dir()
                .expect("Can't access data dir")
                .join("haqor/library"),
        };

        Library { base_path }
    }
}

impl Library {
    pub fn get_library(base_path: &Path) -> Library {
        Library {
            base_path: base_path.to_path_buf(),
        }
    }

    pub fn get_bible(&self, name: &str) -> Bible {
        let name: String = name.to_string();
        let file_path: PathBuf = self.base_path.join(format!("{}.bbl.mybible.gz", name));

        log::info!(
            "Loading bible '{}' from '{:?}' exists '{}'",
            name,
            file_path,
            file_path.exists()
        );

        Bible { name, file_path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_library() {
        let lib = Library::default();

        assert_eq!(lib.base_path, Path::new("~/.haqor/library/"));
    }

    #[test]
    fn test_get_library() {
        let lib = Library::get_library(Path::new("../test_library"));

        assert_eq!(lib.base_path, Path::new("../test_library"));
    }

    #[test]
    fn test_get_bible() {
        let lib = Library::default();
        let bible = lib.get_bible("test_bible");

        assert_eq!(bible.name, "Test Bible");
        assert_eq!(bible.file_name, "file_path");
    }

    /*
    #[test]
    fn get_bible_desciption(){
        let result = get_bible_desciption("test_bible");
    }*/
}
