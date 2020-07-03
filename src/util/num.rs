use std::vec::Vec;

use rand::{thread_rng, Rng};

// const NUMERIC: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

pub fn rand_numbers<T, N>(low: N, high: N, amount: usize) -> Vec<T>
where
    T: rand::distributions::uniform::SampleUniform,
    N: rand::distributions::uniform::SampleBorrow<T> + Copy,
{
    let mut d = Vec::<T>::with_capacity(amount);
    let mut rng = thread_rng();
    for _n in 0..amount {
        d.push(rng.gen_range(low, high))
    }
    d
}
