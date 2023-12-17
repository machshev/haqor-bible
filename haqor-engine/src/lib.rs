// Load text from bible verse (SQLite)

use std::env;
use std::fs::File;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Bible {
    pub name: String,
    file_path: PathBuf,
}

#[derive(Debug)]
pub struct ResourceRepo {
    pub name: String,
    url: String,
}

impl Default for ResourceRepo {
    fn default() -> Self {
        ResourceRepo {
            name: "mysword".to_string(),
            url: "https://mysword-bible.info/download/getfile.php?file=".to_string(),
        }
    }
}

impl ResourceRepo {
    fn fetch_bible(&self, library: Library, name: &str) {
        let bible_url = format!("{}{}.bbl.mybible.gz", self.url, name);

        let resp = reqwest::blocking::get(bible_url).expect("request failed");
        let body = resp.text().expect("body invalid");

        library.save_bible(name, &body);
    }
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

    pub fn save_bible(&self, file_name: &str, content: &str) {
        let mut out = File::create(self.base_path.join(file_name)).expect("failed to create file");
        io::copy(&mut content.as_bytes(), &mut out).expect("failed to copy content");
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

        assert_eq!(
            lib.base_path,
            dirs::data_dir()
                .expect("Can't access data dir")
                .join("haqor/library/")
        );
    }

    #[test]
    fn test_get_library() {
        let lib = Library::get_library(Path::new("../test_library"));

        assert_eq!(lib.base_path, Path::new("../test_library"));
    }

    #[test]
    fn test_default_resource_repo() {
        let repo = ResourceRepo::default();

        assert_eq!(repo.name, "mysword");
        assert_eq!(
            repo.url,
            "https://mysword-bible.info/download/getfile.php?file="
        );
    }

    #[test]
    fn test_resource_repo_fetch_bible() {
        let repo = ResourceRepo::default();
        let lib = Library::default();

        repo.fetch_bible(lib, "kjv");
    }

    #[test]
    fn test_get_bible() {
        let lib = Library::default();
        let bible = lib.get_bible("test_bible");

        assert_eq!(bible.name, "test_bible");
        assert_eq!(
            bible.file_path,
            dirs::data_dir()
                .expect("Can't access data dir")
                .join("haqor/library/test_bible.bbl.mybible.gz")
        );
    }

    /*
    #[test]
    fn get_bible_desciption(){
        let result = get_bible_desciption("test_bible");
    }*/
}
