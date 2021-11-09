# Changelog

## 1.0.0
* Remove `Item` type parameter from `AdjacentPairs` and use `Iterator::Item` instead.
* `#![no_std]` support
* Implement additional traits:
  * `ExactSizeIterator`
  * `FusedIterator`
  * `Clone`
  * `Debug`
* Implement `size_hint`
* Introduce MSRV (minimum rust version) of `1.31`
