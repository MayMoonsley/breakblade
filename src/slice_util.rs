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

pub trait SkipPredicate<T> {
    // must have delay + 1 consecutive predicate hits to skip
    // starts taken regions at first predicate fail - predelay
    // must have hold predicate misses to stop skipping
    // these params can avoid cutting off transients
    fn skip_predicate_with_delay<F: Fn(&T) -> bool>(&self, predicate: F, predelay: usize, hold: usize, delay: usize) -> Vec<&[T]>;

    fn skip_predicate<F: Fn(&T) -> bool>(&self, predicate: F) -> Vec<&[T]> {
        self.skip_predicate_with_delay(predicate, 0, 1, 0)
    }
}

impl<T> SkipPredicate<T> for &[T] {
    fn skip_predicate_with_delay<F: Fn(&T) -> bool>(&self, predicate: F, predelay: usize, hold: usize, delay: usize) -> Vec<&[T]> {
        let mut predicate_hits = 0;
        let mut predicate_misses = 0;
        let mut slice_start = 0;
        // if first element trips the predicate, should exclude until we stop tripping the predicate
        let mut skipping = predicate(&self[0]);
        let mut acc = vec![];
        for i in 0..self.len() {
            if skipping {
                // if the predicate stops being true, stop skipping
                if !predicate(&self[i]) {
                    predicate_misses += 1;
                    if predicate_misses >= hold {
                        skipping = false;
                        slice_start = i.saturating_sub(predelay);
                        predicate_hits = 0;
                    }
                } else {
                    predicate_misses = 0;
                }
            } else {
                if predicate(&self[i]) {
                    predicate_hits += 1;
                    if predicate_hits > delay {
                        acc.push(&self[slice_start..i]);
                        skipping = true;
                        predicate_misses = 0;
                    }
                }
            }
        }
        // include the last little tail portion
        if !skipping {
            acc.push(&self[slice_start..]);
        }
        acc
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

    #[test]
    fn test_skip_predicate_initial_skip() {
        let slice: &[usize] = &[0, 0, 0, 0, 1, 0, 0, 0, 2, 3, 4, 0, 0, 5, 6];
        let result = slice.skip_predicate(|&x| x == 0);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &[1]);
        assert_eq!(result[1], &[2, 3, 4]);
        assert_eq!(result[2], &[5, 6]);
    }

    #[test]
    fn test_skip_predicate_no_delay() {
        let slice: &[usize] = &[1, 0, 0, 0, 2, 3, 4, 0, 0, 5, 6];
        let result = slice.skip_predicate(|&x| x == 0);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &[1]);
        assert_eq!(result[1], &[2, 3, 4]);
        assert_eq!(result[2], &[5, 6]);
    }

    #[test]
    fn test_skip_predicate_with_delay() {
        let slice: &[usize] = &[1, 0, 0, 0, 2, 3, 4, 0, 0, 5, 6];
        let result = slice.skip_predicate_with_delay(|&x| x == 0, 0, 0, 1);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &[1, 0]);
        assert_eq!(result[1], &[2, 3, 4, 0]);
        assert_eq!(result[2], &[5, 6]);
    }

    #[test]
    fn test_skip_predicate_with_predelay() {
        let slice: &[usize] = &[1, 0, 0, 0, 2, 3, 4, 0, 0, 5, 6];
        let result = slice.skip_predicate_with_delay(|&x| x == 0, 1, 0, 0);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &[1]);
        assert_eq!(result[1], &[0, 2, 3, 4]);
        assert_eq!(result[2], &[0, 5, 6]);
    }
}