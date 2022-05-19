// https://youtu.be/yozQ9C69pNs?t=1831

pub fn my_flatten<I>(iter: I) -> Flatten<I> {
    Flatten::new(iter)
}

pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    pub fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().and_then(|i| i.into_iter().next())
    }
}

#[cfg(test)]
mod tests {
    use super::my_flatten;

    #[test]
    fn empty() {
        assert_eq!(my_flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(my_flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(my_flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }
}
