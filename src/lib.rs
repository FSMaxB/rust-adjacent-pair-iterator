#[derive(Clone)]
pub struct AdjacentPairs<IteratorType: Iterator> {
	iterator: IteratorType,
	last_item: Option<IteratorType::Item>,
}

impl<IteratorType: Iterator> AdjacentPairs<IteratorType> {
	fn new(iterator: IteratorType) -> AdjacentPairs<IteratorType> {
		AdjacentPairs {
			iterator,
			last_item: None,
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
}

pub trait AdjacentPairIterator {
	type Iterator: Iterator;

	fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator>;
}

impl<IteratorType> AdjacentPairIterator for IteratorType
where
	IteratorType: Iterator,
	IteratorType::Item: Clone,
{
	type Iterator = Self;

	fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator> {
		AdjacentPairs::new(self)
	}
}

#[cfg(test)]
mod tests {
	use crate::AdjacentPairIterator;

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
		let mut iterator = vector.into_iter().adjacent_pairs();

		assert_eq!(Some((1, 2)), iterator.next());
		assert_eq!(Some((2, 3)), iterator.next());
		assert_eq!(None, iterator.next());
	}
}
