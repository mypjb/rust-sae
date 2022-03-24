use crate::constants::*;
use regex::Regex;
/// <code>String</code> extension method
pub trait StringExtension {
    /// <code>String</code> is <strong>empty</strong> or <strong>white space</strong>
    fn is_empty_or_white_space(&self) -> bool;
}

impl StringExtension for str {
    /// # Examples
    /// ```ignore
    /// let mut str = String::new();
    /// // empty == true
    /// assert!(str.is_empty_or_white_space());
    /// // "  " == true
    /// str.push_str("  ");
    /// assert!(str.is_empty_or_white_space());
    /// // "hello" == false
    /// str.push_str("hello");
    /// assert!(!str.is_empty_or_white_space());
    /// ```
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
