use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq)]
pub struct Story {
    title: String,
    characters: HashMap<String,String>,
    current_dialog: u64,
    description: String
}

impl Story {
    pub fn new() -> Story {
        Story{
            title : "".to_string(),
            characters :HashMap::new(),
            current_dialog :0,
            description: "".to_string()
        }
    }

    pub fn is_new_story(&self) -> bool {
        return self  == &Story::new();
    }

    pub fn check_continuity( &self, dialog:u64 )->bool {
        return self.current_dialog == dialog
    }

    pub fn set_title( &mut self, title:&String )->Result<(),()>{
        self.title = title.to_string();
        Ok(())
    }

    pub fn set_description( &mut self, description:&String )->Result<(),()>{
        self.description = description.to_string();
        Ok(())
    }

    pub fn add_character( &mut self, name:String, tag:String)-> Result<(),String>{
        self.characters.insert(tag, name);
        Ok(())
    }
}
