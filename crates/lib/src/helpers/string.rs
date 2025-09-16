extern crate alloc;

use alloc::string::String;

pub trait StripPrefixOrSelf {
	fn strip_prefix_or_self<P: AsRef<str>>(&self, prefix: P) -> &str;
}

impl StripPrefixOrSelf for str {
	fn strip_prefix_or_self<P: AsRef<str>>(&self, prefix: P) -> &str {
		self.strip_prefix(prefix.as_ref()).unwrap_or(self)
	}
}

pub trait PlaintText {
	fn escape_markdown(&self) -> String;
}

impl<S: AsRef<str>> PlaintText for S {
	fn escape_markdown(&self) -> String {
		let mut markdown = String::new();
		for char in self.as_ref().chars() {
			if char.is_ascii_punctuation() {
				markdown.push('\\');
			}
			markdown.push(char);
		}
		markdown.replace("\r\n", "\n").replace('\n', "  \n")
	}
}
