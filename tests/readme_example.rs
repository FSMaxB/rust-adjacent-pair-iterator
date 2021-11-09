use adjacent_pair_iterator::AdjacentPairIterator;
use std::fmt::{Display, Formatter};

struct ReadmeExample;

impl Display for ReadmeExample {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		let vector = vec![1, 2, 3, 4];
		for pair in vector.adjacent_pairs() {
			writeln!(formatter, "{:?}", pair)?;
		}
		Ok(())
	}
}

#[test]
fn readme_example_output_is_as_expected() {
	let expected_output = r#"(1, 2)
(2, 3)
(3, 4)
"#;
	assert_eq!(expected_output, ReadmeExample.to_string());
}
