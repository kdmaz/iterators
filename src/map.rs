pub struct Map<T, F> {
    iter: T,
    f: F,
}

impl<T, F> Map<T, F> {
    fn new(iter: T, f: F) -> Map<T, F> {
        Map { iter, f }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn multiply() {
        let nums = [1, 2, 3];
        let multiplied = nums.into_iter().map(|num| num * num).collect::<Vec<i32>>();
        assert_eq!(multiplied, [1, 4, 9]);
    }
}
