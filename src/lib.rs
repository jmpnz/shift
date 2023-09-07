mod stateless_shuffle {
    #[derive(Debug, Clone, Copy)]
    pub struct ShuffleIter {
        round_count: u32,
        half_index_bits: u32,
        half_index_bits_mask: u32,
        seed: u32,
    }

    impl ShuffleIter {
        // Create a new stateless shuffle iterator.
        pub fn new(round_count: u32, seed: u32, bits: u32) -> Self {
            ShuffleIter {
                round_count: round_count,
                half_index_bits: bits / 2,
                half_index_bits_mask: (1 << (bits / 2)) - 1,
                seed: seed,
            }
        }

        pub fn to_shuffled_index(&self, index: u32) -> u32 {
            self.encrypt(index)
        }

        pub fn from_shuffled_index(&self, index: u32) -> u32 {
            self.decrypt(index)
        }

        fn encrypt(&self, index: u32) -> u32 {
            let mut left = index >> self.half_index_bits;
            let mut right = index & self.half_index_bits_mask;

            for _ in 0..self.round_count {
                let new_left = right;
                let new_right = left ^ self.round_func(right);
                left = new_left;
                right = new_right;
            }

            (left << self.half_index_bits) | right
        }

        fn decrypt(&self, index: u32) -> u32 {
            let mut left = index >> self.half_index_bits;
            let mut right = index & self.half_index_bits_mask;

            for _ in 0..self.round_count {
                let new_right = left;
                let new_left = right ^ self.round_func(left);
                left = new_left;
                right = new_right;
            }
            (left << self.half_index_bits) | right
        }

        fn round_func(&self, x: u32) -> u32 {
            Self::pcg_hash(x ^ self.seed) & self.half_index_bits_mask
        }

        // PCG hash function ported from github.com/SEED-EA/O1ShufflingGrouping
        // for more details see the following references :
        // https://www.pcg-random.org/
        // https://www.reedbeta.com/blog/hash-functions-for-gpu-rendering/
        fn pcg_hash(input: u32) -> u32 {
            let state = input.wrapping_mul(47796405).wrapping_add(2891336453);
            let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277803737);
            (word >> 22) ^ word
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stateless_shuffle::ShuffleIter;

    #[test]
    fn can_shuffle_16_things_3_times() {
        let seed = 42;
        let index_bits = 4;
        let round_count = 4;
        let shuffler = ShuffleIter::new(round_count, seed, index_bits);

        for _ in 0..4 {
            for index in 0..16 {
                let shuffled_index = shuffler.to_shuffled_index(index);
                let unshuffled_index = shuffler.from_shuffled_index(shuffled_index);

                assert_eq!(index, unshuffled_index);
            }
        }
    }

    #[test]
    fn can_shuffle_12_things_3_times() {
        let seed = 42;
        let index_bits = 4;
        let round_count = 4;
        let shuffler = ShuffleIter::new(round_count, seed, index_bits);

        for _ in 0..4 {
            for index in 0..16 {
                let shuffled_index = shuffler.to_shuffled_index(index);

                if shuffled_index >= 12 {
                    continue;
                }

                let unshuffled_index = shuffler.from_shuffled_index(shuffled_index);

                assert_eq!(index, unshuffled_index);
            }
        }
    }
}
