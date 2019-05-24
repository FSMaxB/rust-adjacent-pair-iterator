pub struct Pairs<IteratorType: Iterator<Item = ItemType>, ItemType: Copy> {
    iterator: IteratorType,
    last_item: Option<ItemType>,
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Copy> Pairs<IteratorType, ItemType> {
    fn new(iterator: IteratorType) -> Pairs<IteratorType, ItemType> {
        Pairs {
            iterator,
            last_item: None,
        }
    }
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Copy> Iterator for Pairs<IteratorType, ItemType> {
    type Item = (ItemType, ItemType);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub trait PairIterator {
    type Item: Copy;
    type Iterator: Iterator<Item = Self::Item>;

    fn pairs(self) -> Pairs<Self::Iterator, Self::Item>;
}

impl<IteratorType: Iterator<Item = ItemType>, ItemType: Copy> PairIterator for IteratorType {
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
}
