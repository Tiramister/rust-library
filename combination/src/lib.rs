use std::ops::{Div, Mul};

pub struct Combination<T> {
    factorials: Vec<T>,
    factorial_invs: Vec<T>,
}

impl<T> Combination<T>
where
    T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
{
    pub fn new() -> Self {
        Combination {
            factorials: vec![T::from(1)],
            factorial_invs: vec![T::from(1)],
        }
    }

    fn extend(&mut self, n: usize) {
        let prev_n = self.factorials.len();
        if n <= prev_n {
            return;
        }

        self.factorials.resize(n, T::from(0));
        for i in prev_n..n {
            self.factorials[i] = self.factorials[i - 1].clone() * T::from(i);
        }

        self.factorial_invs.resize(n, T::from(0));
        self.factorial_invs[n - 1] = T::from(1) / self.factorials[n - 1].clone();
        for i in (prev_n..(n - 1)).rev() {
            self.factorial_invs[i] = self.factorial_invs[i + 1].clone() * T::from(i + 1);
        }
    }

    pub fn factorial(&mut self, n: usize) -> T {
        self.extend(n + 1);
        self.factorials[n].clone()
    }
    pub fn factorial_inv(&mut self, n: usize) -> T {
        self.extend(n + 1);
        self.factorial_invs[n].clone()
    }
    pub fn binomial(&mut self, n: usize, m: usize) -> T {
        if n < m {
            T::from(0)
        } else {
            self.factorial(n) * self.factorial_inv(m) * self.factorial_inv(n - m)
        }
    }
}
