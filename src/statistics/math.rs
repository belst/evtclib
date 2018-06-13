//! This module provides some basic mathematical structures.

/// A semigroup.
///
/// This trait lets you combine elements by a binary operation.
pub trait Semigroup {
    fn combine(&self, other: &Self) -> Self;
}

/// A monoid.
///
/// Extends the semigroup with a "neutral" element.
///
/// # Laws
///
/// ```raw
/// mempty.combine(x) == x
/// x.combine(mempty) == x
/// ```
pub trait Monoid: Semigroup {
    fn mempty() -> Self;
}

#[derive(Debug, Clone)]
struct Record<X, T, D> {
    x: X,
    tag: T,
    data: D,
}

/// A function that records tagged data points.
///
/// This represents a "function" as a list of increases at soem discrete points.
/// Think about it as a generalized damage log. Increases can be tagged by some
/// arbitrary data, for example which the agent ID, the skill ID, the target,
/// ...
///
/// This offers methods to get the value at a specific point (by "summing up"
/// all increments before that point), between two points and in total. It also
/// offers variants that allow you to filter the increments by their tag.
///
/// Type parameters:
///
/// * `X` domain of the function. Must have a defined `Ord`ering.
/// * `T` tag for each data point. Can be arbitrary.
/// * `D` actual data. Must be [`Monoid`](trait.Monoid.html), so that it can be
///   summed up.
#[derive(Clone)]
pub struct RecordFunc<X, T, D> {
    data: Vec<Record<X, T, D>>,
}

impl<X, T, D> RecordFunc<X, T, D>
where
    X: Ord,
    D: Monoid,
{
    /// Create a new `RecordFunc`.
    pub fn new() -> Self {
        RecordFunc { data: Vec::new() }
    }

    /// Insert a data point into the record func.
    ///
    /// Note that you should supply the *increment*, not the *absolute value*!
    pub fn insert(&mut self, x: X, tag: T, data: D) {
        // Usually, the list will be built up in order, which means we can
        // always append to the end. Check for this special case to make it
        // faster.
        if self.data.last().map(|r| r.x < x).unwrap_or(true) {
            self.data.push(Record { x, tag, data });
        } else {
            let index = match self.data.binary_search_by(|r| r.x.cmp(&x)) {
                Ok(i) => i,
                Err(i) => i,
            };
            self.data.insert(index, Record { x, tag, data });
        }
        //self.data.sort_by(|a, b| a.x.cmp(&b.x));
    }

    /// Get the amount of data points saved.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the absolute value at the specific point.
    #[inline]
    pub fn get(&self, x: &X) -> D {
        self.get_only(x, |_| true)
    }

    /// Get the absolute value at the specific point by only considering
    /// increments where the predicate holds.
    pub fn get_only<F: FnMut(&T) -> bool>(&self, x: &X, mut predicate: F) -> D {
        self.data
            .iter()
            .take_while(|record| record.x <= *x)
            .filter(|record| predicate(&record.tag))
            .fold(D::mempty(), |a, b| a.combine(&b.data))
    }

    /// Get the increments between the two given points.
    #[inline]
    pub fn between(&self, a: &X, b: &X) -> D {
        self.between_only(a, b, |_| true)
    }

    /// Get the increments between the two given points by only considering
    /// increments where the predicate holds.
    pub fn between_only<F: FnMut(&T) -> bool>(&self, a: &X, b: &X, mut predicate: F) -> D {
        self.data
            .iter()
            .skip_while(|record| record.x < *a)
            .take_while(|record| record.x <= *b)
            .filter(|record| predicate(&record.tag))
            .fold(D::mempty(), |a, b| a.combine(&b.data))
    }

    /// Get the sum of all increments.
    #[inline]
    pub fn tally(&self) -> D {
        self.tally_only(|_| true)
    }

    /// Get the sum of all increments by only considering increments where the
    /// predicate holds.
    pub fn tally_only<F: FnMut(&T) -> bool>(&self, mut predicate: F) -> D {
        self.data
            .iter()
            .filter(|record| predicate(&record.tag))
            .fold(D::mempty(), |a, b| a.combine(&b.data))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Integer(u32);

    impl Semigroup for Integer {
        fn combine(&self, other: &Self) -> Self {
            Integer(self.0 + other.0)
        }
    }

    impl Monoid for Integer {
        fn mempty() -> Self {
            Integer(0)
        }
    }

    fn create() -> RecordFunc<u32, u8, Integer> {
        let mut result = RecordFunc::new();

        result.insert(6, 1, Integer(6));
        result.insert(4, 0, Integer(5));
        result.insert(0, 1, Integer(3));
        result.insert(2, 0, Integer(4));

        result
    }

    #[test]
    fn recordfunc_get() {
        let rf = create();

        assert_eq!(rf.get(&3), Integer(7));
        assert_eq!(rf.get(&4), Integer(12));
    }

    #[test]
    fn recordfunc_get_only() {
        let rf = create();

        assert_eq!(rf.get_only(&3, |t| *t == 0), Integer(4));
    }

    #[test]
    fn recordfunc_between() {
        let rf = create();

        assert_eq!(rf.between(&1, &5), Integer(9));
    }
}
