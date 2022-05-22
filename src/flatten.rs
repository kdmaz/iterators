pub struct Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    outer_iter: T,
    inner_iter: Option<<T::Item as IntoIterator>::IntoIter>,
}

impl<T> Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    fn new(iter: T) -> Self {
        Flatten {
            outer_iter: iter,
            inner_iter: None,
        }
    }
}

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
        Flatten::new(self)
    }
}

impl<T> Iterator for Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    type Item = <T::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = self.inner_iter.as_mut() {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
            }

            let next_inner_iter = self.outer_iter.next()?.into_iter();
            self.inner_iter = Some(next_inner_iter);
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
