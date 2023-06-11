// Load text from bible verse (SQLite)

pub struct Bible {
    pub name: String,
    pub file_path: String,
}

impl Bible {
    
    pub fn get_bible(name: &str) -> Bible {
        let file_path: String = "file_path".into();

        log::info!("Loading bible '{}'", name);
        
        Bible {name: "Test Bible".to_string(), file_path}
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bible(){
        let bible = Bible::get_bible("test_bible");

        assert_eq!(bible.name, "Test Bible");
        assert_eq!(bible.file_path, "file_path");
    }

    /*
    #[test]
    fn get_bible_desciption(){
        let result = get_bible_desciption("test_bible");
    }*/
}
