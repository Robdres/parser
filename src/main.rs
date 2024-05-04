pub mod parser{
    mod story;
    use crate::parser::story::Story;
    use std::fs::File;
    use std::io::{BufRead, BufReader};


    //#[derive(Debug)]
    //pub struct Dialog {
    //    character: String,
    //    text: String,
    //}

    pub fn get_story_buffer( file: &str ) ->  BufReader<File>{
        return BufReader::new(File::open(file).expect("Error reading file"));
    }

    pub fn get_line( _reader: &mut BufReader<File>, line:&mut String ) -> bool {
        let mut placeholder = String::new();
        let _ = _reader.read_line(&mut placeholder);

        //handles space between sections
        if placeholder.starts_with("\n") {
            let _aux = &placeholder;
            let _ = _reader.read_line(&mut placeholder);
            if placeholder.starts_with("##") {
                *line = placeholder;
                return true
            }else if placeholder.starts_with("\n"){
                *line = "\n".to_string();
                return true;
            }
        }
        if placeholder.is_empty() {
            return false
        }
        *line = placeholder;
        true
    }

    pub fn get_story( _reader: &mut BufReader<File> ) -> Story {

        let mut story = Story::new();
        let mut current_read_line:String = String::new();

        //setting up title
        get_line(_reader, &mut current_read_line ).to_string();
        assert!(current_read_line.starts_with("##"));
        let _ = story.set_title( &current_read_line[2..].to_string() );

        //setting up description
        let mut _des = String::new();
        loop {
            get_line(_reader, &mut current_read_line ).to_string();
            if current_read_line.starts_with("##") || current_read_line.starts_with("\n"){
                break
            }
            _des+=&current_read_line;
        }

        let _ = story.set_description( &_des );

        story
    }

    //pub fn get_next_dialog( reader: &BufReader<File> ) -> Dialog{
    //    todo!()
    //}
}

use std::fs::File;
//use std::io::prelude::*;
use std::io::BufReader;

use crate::parser::get_story;

fn main() {
    let mut buffer: BufReader<File> = parser::get_story_buffer("../story/chapter_1.st" );
    let _story = get_story( &mut buffer );
    eprintln!("story = {:#?}", _story);
}

