pub trait FlattenExt: Iterator + Sized {
    fn flatten2(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> FlattenExt for T
where
    T: Iterator,
{
    fn flatten2(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        Flatten::new(self.into_iter())
    }
}

pub struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    outer: I,
    inner: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    pub fn new(iter: I) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = self.inner.as_mut() {
                if let Some(i) = inner_iter.into_iter().next() {
                    return Some(i);
                }

                self.inner = None;
            }

            let next_inner_item = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FlattenExt;

    #[test]
    fn empty() {
        let iter: std::vec::IntoIter<Vec<&str>> = vec![].into_iter();
        assert_eq!(iter.flatten2().count(), 0);
    }

    #[test]
    fn empty_wide() {
        let iter: std::vec::IntoIter<Vec<&str>> = vec![vec![], vec![], vec![]].into_iter();
        assert_eq!(iter.flatten2().count(), 0);
    }

    #[test]
    fn one() {
        let iter = vec![vec!["a"]].into_iter();
        assert_eq!(iter.flatten2().count(), 1);
    }

    #[test]
    fn two() {
        let iter = vec![vec!["a", "b"]].into_iter();
        assert_eq!(iter.flatten2().count(), 2);
    }

    #[test]
    fn two_wide() {
        let iter = vec![vec!["a"], vec!["b"]].into_iter();
        assert_eq!(iter.flatten2().count(), 2);
    }
}
