use std::fmt::Debug;

pub struct DebugIter<T> {
    i: T,
}

impl<T> Iterator for DebugIter<T>
where
    T: Iterator,
    T::Item: Debug,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.i.next()?;
        println!("{:?}", r);
        Some(r)
    }
}

pub trait DbgIterator<T> {
    fn dbg(self) -> DebugIter<T>;
}

impl<T> DbgIterator<T> for T
where
    T: Iterator,
    T::Item: Debug,
{
    fn dbg(self) -> DebugIter<T> {
        DebugIter { i: self }
    }
}
