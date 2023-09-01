// Load text from bible verse (SQLite)

use std::path::{PathBuf, Path};

pub struct Bible {
    pub name: String,
    file_name: String,
}


pub struct Library {
    base_path: PathBuf,
}

impl Default for Library {
    fn default() -> Self {
        Library {base_path: PathBuf::from("~/.haqor/library/")}
    }
}


impl Library {

    pub fn get_library(base_path: &Path) -> Library{
        Library {base_path: base_path.to_path_buf()}
    }

    pub fn get_bible(&self, name: &str) -> Bible {
        let file_name: String = "file_path".into();

        log::info!("Loading bible '{}'", name);
        
        Bible {name: "Test Bible".to_string(), file_name}
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_library(){
        let lib = Library::default();

        assert_eq!(lib.base_path, Path::new("~/.haqor/library/"));
    }

    #[test]
    fn test_get_library(){
        let lib = Library::get_library(Path::new("../test_library"));

        assert_eq!(lib.base_path, Path::new("../test_library"));
    }


    
    #[test]
    fn test_get_bible(){
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
