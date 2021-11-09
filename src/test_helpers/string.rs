use core::fmt::{Arguments, Debug, Formatter, Write};
use core::str::from_utf8;

pub struct NoStdString {
	bytes: [u8; 4096],
	length: usize,
}

impl NoStdString {
	pub fn format(arguments: Arguments) -> Result<Self, core::fmt::Error> {
		let mut text_buffer = NoStdString::default();
		text_buffer.write_fmt(arguments)?;
		Ok(text_buffer)
	}

	pub fn removing(mut self, pattern: u8) -> Self {
		let mut source_index = 0;
		let mut destination_index = 0;
		while source_index < self.length {
			if self.bytes[source_index] == pattern {
				source_index += 1;
				continue;
			}

			self.bytes[destination_index] = self.bytes[source_index];
			destination_index += 1;
			source_index += 1;
		}

		self.length = destination_index;

		self
	}
}

impl Debug for NoStdString {
	fn fmt(&self, formatter: &mut Formatter) -> core::fmt::Result {
		formatter.debug_tuple("NoStdTextBuffer").field(&self.as_ref()).finish()
	}
}

impl Default for NoStdString {
	fn default() -> Self {
		Self {
			bytes: [0; 4096],
			length: 0,
		}
	}
}

impl AsRef<str> for NoStdString {
	fn as_ref(&self) -> &str {
		from_utf8(&self.bytes[0..self.length]).expect("Invalid UTF-8 in NoStdString, this must not happen")
	}
}

impl PartialEq<&str> for NoStdString {
	fn eq(&self, &other: &&str) -> bool {
		self.as_ref() == other
	}
}

impl PartialEq<NoStdString> for &str {
	fn eq(&self, other: &NoStdString) -> bool {
		other == self
	}
}

impl core::fmt::Write for NoStdString {
	fn write_str(&mut self, str: &str) -> core::fmt::Result {
		let new_length = self.length + str.len();
		if new_length > self.bytes.len() {
			return Err(Default::default());
		}

		(&mut self.bytes[self.length..new_length]).copy_from_slice(str.as_bytes());
		self.length = new_length;
		Ok(())
	}

	fn write_char(&mut self, codepoint: char) -> core::fmt::Result {
		let new_length = self.length + codepoint.len_utf8();
		if new_length > self.bytes.len() {
			return Err(Default::default());
		}

		codepoint.encode_utf8(&mut self.bytes[self.length..new_length]);
		self.length = new_length;
		Ok(())
	}
}
