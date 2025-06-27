pub trait StripPrefixOrSelf {
	fn strip_prefix_or_self<P: AsRef<str>>(&self, prefix: P) -> &str;
}

impl StripPrefixOrSelf for str {
	fn strip_prefix_or_self<P: AsRef<str>>(&self, prefix: P) -> &str {
		self.strip_prefix(prefix.as_ref()).unwrap_or(self)
	}
}
