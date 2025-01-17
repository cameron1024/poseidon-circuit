use core::fmt;
use std::iter::FusedIterator;

pub fn group_by<T, F>(slice: &[T], predicate: F) -> GroupBy<'_, T, F>
where
    F: FnMut(&T, &T) -> bool,
{
    GroupBy { slice, predicate }
}

pub struct GroupBy<'a, T: 'a, P> {
    slice: &'a [T],
    predicate: P,
}

impl<'a, T: 'a, P> GroupBy<'a, T, P> {
    pub(super) fn new(slice: &'a [T], predicate: P) -> Self {
        GroupBy { slice, predicate }
    }
}

impl<'a, T: 'a, P> Iterator for GroupBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next() {
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let (head, tail) = self.slice.split_at(len);
            self.slice = tail;
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() { (0, Some(0)) } else { (1, Some(self.slice.len())) }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T: 'a, P> DoubleEndedIterator for GroupBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next_back() {
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let (head, tail) = self.slice.split_at(self.slice.len() - len);
            self.slice = head;
            Some(tail)
        }
    }
}

impl<'a, T: 'a, P> FusedIterator for GroupBy<'a, T, P> where P: FnMut(&T, &T) -> bool {}

impl<'a, T: 'a + fmt::Debug, P> fmt::Debug for GroupBy<'a, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroupBy").field("slice", &self.slice).finish()
    }
}

