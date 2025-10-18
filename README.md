# Self-descriptive numbers

This program generates self descriptive numbers with the specified pair length.

The key insights here are that
* For a SDN with n pairs and k unique digits, the repetitions must be a partition of n with k parts, where no part is greater than the largest unique digit, because the sum of repetitions (where 1 indicates that a pair is not repeated), must be equal to the number of pairs n.
* For a SDN with n pairs and k unique digits, the descriptors (left components of pairs) must be a partition of 2n with k parts, where no part is greater than
the largest unique digit, because the sum of descriptors must be equal to the
total number of digits.
* For each repetition-descriptor pair, the repetition must be less than or equal
to the descriptor. This implies that the sorted repetitions must be less than or equal to the sorted descriptors.

This program efficiently generates possible repetitions and descriptors,
and then generates possible unique pairings.

## Usage

```
cargo run --release 37
```