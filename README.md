# adjacent-pair-iterator
A `#![no_std]` library that takes an iterator and turns it into an iterator over adjacent pairs.

## Minimum rust version (MSRV)
This library works with Rust versions since 1.31.

## Example:
```rust
use adjacent_pair_iterator::AdjacentPairIterator;

pub fn main() {
	let vector = vec![1, 2, 3, 4];
	for pair in vector.adjacent_pairs() {
		println!("{:?}", pair);
	}
}
```

Prints:
```
(1, 2)
(2, 3)
(3, 4)
```
