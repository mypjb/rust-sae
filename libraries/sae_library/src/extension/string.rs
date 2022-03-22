use regex::Regex;
use crate::constants::*;

pub trait StringExtension {
    fn is_empty_or_white_space(&self) -> bool;
}

impl StringExtension for String {
    fn is_empty_or_white_space(&self) -> bool {
        match self.is_empty() {
            true => {
                return true;
            }
            false => {
                let re = Regex::new(WHITE_SPACE_REGEX).unwrap();
                return re.is_match(self);
            }
        }
    }
}