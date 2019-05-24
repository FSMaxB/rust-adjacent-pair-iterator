/*
 * adjacent-pair-iterator
 *
 * Copyright (C) 2019 Max Bruckner (FSMaxB)
 *
 * Permission to use, copy, modify, and/or distribute this software for any purpose with or without
 * fee is hereby granted, provided that the above copyright notice and this permission notice appear
 * in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
 * SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
 * AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
 * NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

pub struct AdjacentPairs<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> {
	iterator: IteratorType,
	last_item: Option<ItemType>,
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> AdjacentPairs<IteratorType, ItemType> {
	fn new(iterator: IteratorType) -> AdjacentPairs<IteratorType, ItemType> {
		AdjacentPairs {
			iterator,
			last_item: None,
		}
	}
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> Iterator for AdjacentPairs<IteratorType, ItemType> {
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

pub trait AdjacentPairIterator {
	type Item: Clone;
	type Iterator: Iterator<Item = Self::Item>;

	fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator, Self::Item>;
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Clone> AdjacentPairIterator for IteratorType {
	type Item = ItemType;
	type Iterator = Self;

	fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator, Self::Item> {
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
