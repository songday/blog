use std::vec::Vec;

const NUMERIC: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

pub fn rand_number<T, N>(min: N, max: N) -> T
where
    T: rand::distributions::uniform::SampleUniform,
    N: rand::distributions::uniform::SampleBorrow<T>,
{
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

pub fn rand_numbers(amount: usize) -> Vec<u8> {
    let mut d = Vec::with_capacity(amount);
    for n in 0..amount {
        let n: u8 = rand_number(0, 10);
        d.push(n);
    }
    d
}
