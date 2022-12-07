use std::fs;
use std::io;
use std::path::Path;

pub trait Solution {
    type Ret;
    type Converted;

    fn get_input(path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn solve() -> Self::Ret;

    fn convert(input: &str) -> Self::Converted;
}
