/// there aren't really for loops in rust
/// into iterator is another trait,
///

/// One would need a associated type because,
/// if YOU expect there should be only one implementation
/// for a given type
///
/// trait Service<Request> {
///     fn do(&mut self, r: Request);
/// }
///

pub fn flatten<O>(iter: O) -> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    // <O as Iterator>::Item: IntoIterator,
    // I don't have to do "as Iterator" because there is only
    // one trait bound of O which gives associated type Item
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }

            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        )
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec![1, 2])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec![0], vec![1]].into_iter()).count(), 2);
    }
}
