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
