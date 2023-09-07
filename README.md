# Shift

Rust port of
[SEED-EA/O1ShufflingGrouping](https://github.com/SEED-EA/O1ShufflingGrouping/tree/main).

## Example

```rust

use shift::stateless_shuffle;

fn main() {

    // Ideally use a pseudo-random seed
    let seed = 42;
    // For more information about the parameters read the original blog.
    let index_bits = 4;
    let round_count = 4;
    let shuffler = stateless_shuffle::ShuffleIter::new(round_count, seed, index_bits);

    for _ in 0..8 {
        for index in 0..16 {
            let shuffled_index = shuffler.to_shuffled_index(index);
            let unshuffled_index = shuffler.from_shuffled_index(shuffled_index);

            assert_eq!(index, unshuffled_index);

            println!("Original Index : {index}");
            println!("Shuffled Index : {shuffled_index}");
            println!("Unshuffled Index : {unshuffled_index}");
        }
    }
}

```
