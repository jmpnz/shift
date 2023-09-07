use shift::stateless_shuffle;

fn main() {
    println!("Example of Stateless Shuffling...");
    println!("Let's shuffle 12 indices 4 times...");

    let seed = 42;
    let index_bits = 4;
    let round_count = 4;
    let shuffler = stateless_shuffle::ShuffleIter::new(round_count, seed, index_bits);

    for _ in 0..4 {
        for index in 0..16 {
            let shuffled_index = shuffler.to_shuffled_index(index);

            if shuffled_index >= 12 {
                continue;
            }

            let unshuffled_index = shuffler.from_shuffled_index(shuffled_index);

            assert_eq!(index, unshuffled_index);

            println!("Original Index : {index}");
            println!("Shuffled Index : {shuffled_index}");
            println!("Unshuffled Index : {unshuffled_index}");
        }
    }
}
