pub fn run_length_compression<T>(xs: &[T]) -> Vec<(T, usize)>
where
    T: Eq + Clone,
{
    let mut ret = Vec::<(T, usize)>::new();
    for x in xs {
        if ret.is_empty() || *x != ret.last().unwrap().0 {
            ret.push((x.clone(), 1));
        } else {
            ret.last_mut().unwrap().1 += 1;
        }
    }
    ret
}

pub struct Compressor<T>
where
    T: Ord + Clone,
{
    values: Vec<T>,
}

impl<T> Compressor<T>
where
    T: Ord + Clone,
{
    pub fn new(values: &[T]) -> Self {
        let mut values = values.to_owned();
        values.sort();
        values.dedup();
        Self {
            values: values.to_owned(),
        }
    }

    pub fn values(&self) -> Vec<T> {
        self.values.clone()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn value(&self, index: usize) -> T {
        self.values[index].clone()
    }

    pub fn index(&self, value: T) -> Option<usize> {
        // values[l] <= value
        let mut l = 0;
        let mut r = self.values.len();

        while r - l > 1 {
            let mid = (l + r) / 2;
            if self.values[mid] <= value {
                l = mid;
            } else {
                r = mid;
            }
        }

        if self.values[l] == value {
            Some(l)
        } else {
            None
        }
    }
}
