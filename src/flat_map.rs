use crate::{
    flatten::{Flatten, FlattenExt},
    map::{Map, MapExt},
};

pub trait FlatMapExt: Iterator + Sized {
    fn flat_map2<F, U>(self, f: F) -> Flatten<Map<Self, F, U>>
    where
        U: IntoIterator,
        F: FnMut(Self::Item) -> U;
}

impl<T> FlatMapExt for T
where
    T: Iterator,
{
    fn flat_map2<F, U>(self, f: F) -> Flatten<Map<T, F, U>>
    where
        U: IntoIterator,
        F: FnMut(Self::Item) -> U,
    {
        self.map2(f).flatten2()
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
