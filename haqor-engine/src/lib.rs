// Load text from bible verse (SQLite)

pub struct Bible {
    pub name: String,
    file_name: String,
}


pub struct Library {
    base_path: String,
}

impl Default for Library {
    fn default() -> Self {
        Library {base_path: "Default".to_string()}
    }
}


impl Library {

    pub fn get_library(base_path: &str) -> Library{
        Library {base_path: base_path.to_string()}
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

        assert_eq!(lib.base_path, "Default");
    }

    #[test]
    fn test_get_library(){
        let lib = Library::get_library("../test_library");

        assert_eq!(lib.base_path, "../test_library");
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
