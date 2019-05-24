pub struct Pairs<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> {
	iterator: IteratorType,
	last_item: Option<ItemType>,
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> Pairs<IteratorType, ItemType> {
	fn new(iterator: IteratorType) -> Pairs<IteratorType, ItemType> {
		Pairs {
			iterator,
			last_item: None,
		}
	}
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> Iterator for Pairs<IteratorType, ItemType> {
	type Item = (ItemType, ItemType);

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

pub trait PairIterator {
	type Item: Clone;
	type Iterator: Iterator<Item = Self::Item>;

	fn pairs(self) -> Pairs<Self::Iterator, Self::Item>;
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> PairIterator for IteratorType {
	type Item = ItemType;
	type Iterator = Self;

	fn pairs(self) -> Pairs<Self::Iterator, Self::Item> {
		Pairs::new(self)
	}
}

#[cfg(test)]
mod tests {
	use crate::PairIterator;

	#[test]
	fn should_provide_nothing_without_items() {
		let array: [i32; 0] = [];
		let mut iterator = array.iter().pairs();

		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_nothing_for_only_one_input() {
		let array = [1];
		let mut iterator = array.iter().pairs();

		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_pair_for_two_inputs() {
		let array = [1, 2];
		let mut iterator = array.iter().pairs();

		assert_eq!(Some((&1, &2)), iterator.next());
		assert_eq!(None, iterator.next());
	}

	#[test]
	fn should_provide_two_pairs_for_three_inputs() {
		let array = [1, 2, 3];
		let mut iterator = array.iter().pairs();

		assert_eq!(Some((&1, &2)), iterator.next());
		assert_eq!(Some((&2, &3)), iterator.next());
		assert_eq!(None, iterator.next());
	}
}
