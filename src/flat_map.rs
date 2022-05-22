use std::iter::Map;

pub trait FlatMapExt: Iterator + Sized {
    fn flat_map2<F, U>(self, f: F) -> Flatten<Map<Self, F>>
    where
        U: IntoIterator,
        F: FnMut(Self::Item) -> U;
}

impl<T> FlatMapExt for T
where
    T: Iterator,
{
    fn flat_map2<F, U>(self, f: F) -> Flatten<Map<T, F>>
    where
        U: IntoIterator,
        F: FnMut(Self::Item) -> U,
    {
        Flatten::new(self.map(f))
    }
}

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
    pub fn new(iter: T) -> Self {
        Flatten {
            outer_iter: iter,
            inner_iter: None,
        }
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
    use super::FlatMapExt;

    #[test]
    fn words() {
        let words = ["alpha", "beta", "gamma"];

        // chars() returns an iterator
        let merged: String = words.into_iter().flat_map2(|s| s.chars()).collect();
        assert_eq!(merged, "alphabetagamma");
    }
}
