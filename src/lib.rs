use std::fmt::{Debug, Formatter};
use std::iter::FusedIterator;

#[derive(Clone)]
pub struct AdjacentPairs<IteratorType: Iterator> {
	iterator: IteratorType,
	last_item: Option<IteratorType::Item>,
}

impl<IteratorType> AdjacentPairs<IteratorType>
where
	IteratorType: Iterator,
	IteratorType::Item: Clone,
{
	fn new(iterator: IteratorType) -> AdjacentPairs<IteratorType> {
		AdjacentPairs {
			iterator,
			last_item: None,
		}
	}

	fn remaining_pairs_for_given_size(&self, size: usize) -> usize {
		let remaining_elements = size + self.last_item.is_some() as usize;
		if remaining_elements > 0 {
			remaining_elements - 1
		} else {
			0
		}
	}
}

impl<IteratorType> Iterator for AdjacentPairs<IteratorType>
where
	IteratorType: Iterator,
	IteratorType::Item: Clone,
{
	type Item = (IteratorType::Item, IteratorType::Item);

	fn next(&mut self) -> Option<Self::Item> {
		let last_item = match self.last_item.take() {
			Some(item) => item,
			None => self.iterator.next()?,
		};

		let current_item = self.iterator.next()?;
		self.last_item = Some(current_item.clone());
		Some((last_item, current_item))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let (lower, upper) = self.iterator.size_hint();
		(
			self.remaining_pairs_for_given_size(lower),
			upper.map(|upper| self.remaining_pairs_for_given_size(upper)),
		)
	}
}

impl<IteratorType> Debug for AdjacentPairs<IteratorType>
where
	IteratorType: Iterator + Debug,
{
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		formatter
			.debug_struct("AdjacentPairs")
			.field("iterator", &self.iterator)
			.finish()
	}
}

impl<IteratorType> FusedIterator for AdjacentPairs<IteratorType>
where
	IteratorType: FusedIterator,
	IteratorType::Item: Clone,
{
}

impl<IteratorType> ExactSizeIterator for AdjacentPairs<IteratorType>
where
	IteratorType: ExactSizeIterator,
	IteratorType::Item: Clone,
{
}

impl<Iterable> From<Iterable> for AdjacentPairs<Iterable::IntoIter>
where
	Iterable: IntoIterator,
	Iterable::Item: Clone,
{
	fn from(iterable: Iterable) -> Self {
		Self::new(iterable.into_iter())
	}
}

pub trait AdjacentPairIterator {
	type Iterator: Iterator;

	fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator>;
}

impl<Iterable> AdjacentPairIterator for Iterable
where
	Iterable: IntoIterator,
	Iterable::Item: Clone,
{
	type Iterator = <Self as IntoIterator>::IntoIter;

	fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator> {
		self.into()
	}
}

#[cfg(test)]
mod tests {
	use crate::{AdjacentPairIterator, AdjacentPairs};

	#[test]
	fn should_provide_nothing_without_items() {
		let array: [i32; 0] = [];
		let mut iterator = array.iter().adjacent_pairs();

		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_nothing_for_only_one_input() {
		let array = [1];
		let mut iterator = array.iter().adjacent_pairs();

		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_pair_for_two_inputs() {
		let array = [1, 2];
		let mut iterator = array.iter().adjacent_pairs();

		assert_eq!(Some((&1, &2)), iterator.next());
		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_two_pairs_for_three_inputs() {
		let array = [1, 2, 3];
		let mut iterator = array.iter().adjacent_pairs();

		assert_eq!(Some((&1, &2)), iterator.next());
		assert_eq!(Some((&2, &3)), iterator.next());
		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_many_pairs() {
		let array = [1, 2, 3, 4, 5, 6];
		let mut iterator = array.iter().adjacent_pairs();

		assert_eq!(Some((&1, &2)), iterator.next());
		assert_eq!(Some((&2, &3)), iterator.next());
		assert_eq!(Some((&3, &4)), iterator.next());
		assert_eq!(Some((&4, &5)), iterator.next());
		assert_eq!(Some((&5, &6)), iterator.next());
		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_work_with_into_iterator() {
		let vector = vec![1, 2, 3];
		let mut iterator = vector.adjacent_pairs();

		assert_eq!(Some((1, 2)), iterator.next());
		assert_eq!(Some((2, 3)), iterator.next());
		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_update_its_size_hint() {
		let array = [0; 5];
		let mut iterator = array.iter().adjacent_pairs();

		assert_eq!((4, Some(4)), iterator.size_hint());
		iterator.next();
		assert_eq!((3, Some(3)), iterator.size_hint());
		iterator.next();
		assert_eq!((2, Some(2)), iterator.size_hint());
		iterator.next();
		assert_eq!((1, Some(1)), iterator.size_hint());
		iterator.next();
		assert_eq!((0, Some(0)), iterator.size_hint());
		assert!(iterator.next().is_none());
	}

	#[test]
	fn should_debug_print() {
		let array = [1, 2];
		let iterator = array.iter().adjacent_pairs();

		assert_eq!("AdjacentPairs { iterator: Iter([1, 2]) }", format!("{:?}", iterator));

		// NOTE: With stripped commas because this changed between 1.31 and now (1.56)
		let expected_pretty_debug_output = r#"AdjacentPairs {
    iterator: Iter(
        [
            1
            2
        ]
    )
}"#;
		let pretty_debug_output_without_commas = format!("{:#?}", iterator).replace(',', "");
		assert_eq!(expected_pretty_debug_output, pretty_debug_output_without_commas);
	}

	#[test]
	fn should_convert_from_iterable() {
		let vector = vec![1, 2];
		let mut iterator: AdjacentPairs<_> = From::from(vector);
		assert_eq!(Some((1, 2)), iterator.next());
	}
}
