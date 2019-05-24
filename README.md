adjacent-pair-iterator
======================

**An iterator over adjacent pairs in another iterator.**

Usage
-----
```toml
[dependencies]
adjacent-pair-iterator = "0.1.0"
```

```rust
extern crate adjacent_pair_iterator; // 2015 Edition only

use adjacent_pair_iterator::AdjacentPairIterator;

fn main() {
    let array = [1, 2, 3, 4];

    for pair in array.iter().adjacent_pairs() {
        println!("{:?}", pair);
    }
}
```
