pub trait SkipWhile<T> {
    fn skip_while<F: Fn(&T) -> bool>(&self, predicate: F) -> &[T];
}

impl<T> SkipWhile<T> for &[T] {
    fn skip_while<F: Fn(&T) -> bool>(&self, predicate: F) -> &[T] {
        for i in 0..self.len() {
            if !predicate(&self[i]) {
                return &self[i..]
            }
        }
        &[]
    }
}

pub trait SkipFromRightWhile<T> {
    fn skip_from_right_while<F: Fn(&T) -> bool>(&self, predicate: F) -> &[T];
}

impl<T> SkipFromRightWhile<T> for &[T] {
    fn skip_from_right_while<F: Fn(&T) -> bool>(&self, predicate: F) -> &[T] {
        for i in (0..self.len()).rev() {
            if !predicate(&self[i]) {
                return &self[..=i];
            }
        }
        &[]
    }
}

pub trait TakeUntil<T> {
    fn take_until<F: Fn(&T) -> bool>(&self, predicate: F) -> &[T];
}

impl<T> TakeUntil<T> for &[T] {
    fn take_until<F: Fn(&T) -> bool>(&self, predicate: F) -> &[T] {
        for i in 0..self.len() {
            if predicate(&self[i]) {
                return &self[..i]
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_skip_while() {
        let a: &[usize] = &[2, 2, 2, 3, 3, 3, 4, 4, 4];
        let b: &[usize] = a.skip_while(|&x| x == 2);
        assert_eq!(b, [3, 3, 3, 4, 4, 4]);
        let c: &[usize] = b.skip_while(|&x| x == 3);
        assert_eq!(c, [4, 4, 4]);
        assert_eq!(a.skip_while(|&x| x == 2).skip_while(|&x| x == 3), [4, 4, 4]);
    }

    #[test]
    fn test_skip_from_right_while() {
        let slice: &[usize] = &[1, 2, 3, 4, 5, 6];
        assert_eq!(slice.skip_from_right_while(|&x| x > 3), [1, 2, 3]);
    }

    #[test]
    fn test_take_until() {
        let slice: &[usize] = &[1, 2, 3, 4, 5, 6];
        assert_eq!(slice.take_until(|&x| x > 3), [1, 2, 3]);
    }
}