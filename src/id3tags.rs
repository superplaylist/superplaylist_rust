use id3::Tag;
use std::io;

pub struct ID3Tags {}

impl ID3Tags {
    pub fn get_mp3_metadata(file: &str) -> io::Result<String> {
        let tag = Tag::read_from_path(file);
        println!("{:?}", tag);
        Ok(String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_id3() {

    }
}