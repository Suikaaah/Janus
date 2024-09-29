use anyhow::Result;
use std::{collections::LinkedList, fs::File, io::Read, path::Path};

pub fn char_list(text: impl AsRef<str>) -> LinkedList<char> {
    text.as_ref()
        .chars()
        .fold(LinkedList::new(), |mut list, character| {
            list.push_back(character);
            list
        })
}

pub fn read_file(path: impl AsRef<Path>) -> Result<LinkedList<char>> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf.chars().collect())
}
