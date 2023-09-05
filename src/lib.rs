//! Efficiently enumerate integer partitions.
//!
//! This is an implementation of a method described by
//! [Jerome Kelleher](http://jeromekelleher.net/generating-integer-partitions.html),
//! which takes a constant amount of time for each partition.

#![deny(missing_docs)]

/// Iterates over the partitions of a given positive integer.
pub struct Partitions {
    a: Vec<usize>,
    k: usize,
    y: usize,
    next: State,
}

enum State {
    A,
    B { x: usize, l: usize },
}

impl Partitions {
    /// Makes a new iterator.
    #[inline]
    pub fn new(n: usize) -> Partitions {
        Partitions {
            a: vec![0; n + 1],
            k: if n == 0 { 0 } else { 1 },
            y: if n == 0 { 0 } else { n - 1 },
            next: State::A,
        }
    }

    /// Makes a new iterator, trying to avoid allocations.
    ///
    /// Any vector can be passed to this function, since its contents
    /// will be cleared and it will be filled with zeroes, but note
    /// that the vector will still reallocate if its capacity is less
    /// than `n + 1`.
    #[inline]
    pub fn recycle(n: usize, mut vec: Vec<usize>) -> Partitions {
        vec.clear();
        vec.reserve(n + 1);
        for _ in 0..(n + 1) {
            vec.push(0);
        }

        Partitions {
            a: vec,
            k: if n == 0 { 0 } else { 1 },
            y: if n == 0 { 0 } else { n - 1 },
            next: State::A,
        }
    }

    /// Destroys the iterator and returns a vector for further use.
    ///
    /// You only need to call this function if you want to reuse the
    /// vector for something else. Its contents will be in an undefined
    /// state, and so cannot be relied upon.
    #[inline]
    pub fn end(self) -> Vec<usize> {
        self.a
    }
}

impl Iterator for Partitions {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Vec::new();

        match self.next {
            State::A => {
                if self.k == 0 {
                    if self.a.len() == 1 {
                        self.a.pop();
                        return Some(result);
                    } else {
                        return None;
                    }
                } else {
                    self.k -= 1;
                    let x = self.a[self.k] + 1;

                    while 2 * x <= self.y {
                        self.a[self.k] = x;
                        self.y -= x;
                        self.k += 1;
                    }

                    let l = self.k + 1;

                    if x <= self.y {
                        self.a[self.k] = x;
                        self.a[l] = self.y;
                        self.next = State::B { x, l };
                        result.extend_from_slice(&self.a[..self.k + 2]);
                    } else {
                        self.a[self.k] = x + self.y;
                        self.y = x + self.y - 1;
                        result.extend_from_slice(&self.a[..self.k + 1]);
                    }
                }
            }
            State::B { mut x, l } => {
                x += 1;
                self.y -= 1;

                if x <= self.y {
                    self.a[self.k] = x;
                    self.a[l] = self.y;
                    self.next = State::B { x, l };
                    result.extend_from_slice(&self.a[..self.k + 2]);
                } else {
                    self.a[self.k] = x + self.y;
                    self.y = x + self.y - 1;
                    self.next = State::A;
                    result.extend_from_slice(&self.a[..self.k + 1]);
                }
            }
        }

        Some(result)
    }
}

#[test]
fn oeis() {
    //! Tests the first few entries of A000041.

    let tests: &[usize] = &[
        1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792,
        1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883,
        17977, 21637, 26015, 31185, 37338, 44583, 53174, 63261, 75175, 89134, 105558, 124754,
        147273, 173525,
    ];

    for partition in Partitions::new(10) {
        print!("{:?}\n", partition)
    }

    for (i, &n) in tests.iter().enumerate() {
        let p = Partitions::new(i);
        let mut c = 0;

        for partition in p {
            let sum: usize = partition.iter().cloned().sum();
            assert_eq!(sum, i);
            c += 1;
        }

        assert_eq!(c, n);
    }
}
