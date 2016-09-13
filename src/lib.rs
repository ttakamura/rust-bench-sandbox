#![feature(test)]
extern crate test;

const VEC_SIZE: usize = 1000000;

pub fn vec_loop() {
    let mut x: Vec<u32> = vec![1; VEC_SIZE];
    for _ in 0..10 {
        for n in 0..VEC_SIZE {
            x[n] = x[n] + 3;
        }
    }
}

pub fn array_loop() {
    let mut x = [1; 100000];
    for _ in 0..10 {
        for n in 0..100000 {
            x[n] = x[n] + 3;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_vec(b: &mut Bencher) {
        b.iter(|| vec_loop());
    }

    #[bench]
    fn bench_array(b: &mut Bencher) {
        b.iter(|| array_loop());
    }
}
