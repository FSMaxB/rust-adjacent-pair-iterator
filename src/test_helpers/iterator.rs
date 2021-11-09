pub struct NoStdIntoIterator<T>(T, T);

impl<T> From<[T; 2]> for NoStdIntoIterator<T> {
	fn from(array: [T; 2]) -> Self {
		let [first, second] = array;
		NoStdIntoIterator(first, second)
	}
}

impl<T> IntoIterator for NoStdIntoIterator<T> {
	type Item = T;
	type IntoIter = NoStdIterator<T>;

	fn into_iter(self) -> Self::IntoIter {
		let NoStdIntoIterator(first, second) = self;
		NoStdIterator {
			first: Some(first),
			second: Some(second),
		}
	}
}

pub struct NoStdIterator<T> {
	first: Option<T>,
	second: Option<T>,
}

impl<T> Iterator for NoStdIterator<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.first.take().or_else(|| self.second.take())
	}
}
