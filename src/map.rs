pub struct Map<I, F, U>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
{
    iter: I,
    f: F,
}

impl<I, F, U> Map<I, F, U>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
{
    fn new(iter: I, f: F) -> Map<I, F, U>
    where
        I: Iterator,
        F: FnMut(I::Item) -> U,
    {
        Map { iter, f }
    }
}

pub trait MapExt: Iterator + Sized {
    fn map2<F, U>(self, f: F) -> Map<Self, F, U>
    where
        F: FnMut(Self::Item) -> U;
}

impl<I> MapExt for I
where
    I: Iterator,
{
    fn map2<F, U>(self, f: F) -> Map<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
    {
        Map::new(self, f)
    }
}

impl<I, F, U> Iterator for Map<I, F, U>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        Some((self.f)(item))
    }
}

#[cfg(test)]
mod tests {
    use crate::map::MapExt;

    #[test]
    fn multiply() {
        let nums = [1, 2, 3];
        let multiplied = nums.into_iter().map2(|num| num * num).collect::<Vec<i32>>();
        assert_eq!(multiplied, [1, 4, 9]);
    }
}
