use rand::{self, Rng};

#[derive(Debug, Clone)]
pub struct Shuffle<T> {
    coll: Vec<T>,
    len: usize,
    idx: usize,
}

fn shuffle<T>(coll: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    rng.shuffle(coll);
}

impl<T: Clone> Shuffle<T> {
    pub fn new(vec: Vec<T>) -> Shuffle<T> {
        let len = vec.len();
        let mut coll = vec.clone();
        shuffle(&mut coll);
        Shuffle { coll, len, idx: 0 }
    }
}

impl<T: Clone + Eq> Iterator for Shuffle<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.coll[self.idx].clone();
        if self.len == 1 {
            return Some(next_item);
        } else if self.idx == self.len - 1 {
            self.idx = 0;
            shuffle(&mut self.coll);
            while self.coll[0] == next_item {
                shuffle(&mut self.coll);
            }
        } else {
            self.idx += 1;
        }
        Some(next_item)
    }
}

#[cfg(test)]
mod tests {
    use shuffle::Shuffle;
    use itertools::Itertools;
    use quickcheck::TestResult;

    quickcheck! {
        fn shuffles_into_chunks_of_its_length(xs: Vec<u32>) -> TestResult {
            let len = xs.len();
            if len <= 2 { return TestResult::discard() }
            let sum: u32 = xs.iter().sum();
            TestResult::from_bool(
                Shuffle::new(xs)
                .chunks(len).into_iter()
                .take(len)
                .all(|ys| ys.sum::<u32>() == sum))
        }

        fn shuffled_chunks_dont_have_adjacent_duplicates(xs: Vec<String>) -> TestResult {
            let xs = xs.into_iter().unique().collect::<Vec<_>>();
            let len = xs.len();
            if len <= 2 { return TestResult::discard() }
            TestResult::from_bool(
                Shuffle::new(xs)
                .take(len)
                .tuple_windows::<(_, _)>()
                .step(len)
                .all(|(x, y)| x != y))
        }
    }
}
