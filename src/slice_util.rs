pub fn take_until<'a, T, F: Fn(&T) -> bool>(input: &'a[T], predicate: F) -> &'a[T] {
    for i in 0..input.len() {
        if predicate(&input[i]) {
            return &input[..=i]
        }
    }
    &input
}

pub fn take_while<'a, T, F: Fn(&T) -> bool>(input: &'a[T], predicate: F) -> &'a[T] {
    for i in 0..input.len() {
        if !predicate(&input[i]) {
            return &input[..=i]
        }
    }
    &input
}

pub fn skip_while<'a, T, F: Fn(&T) -> bool>(input: &'a[T], predicate: F) -> &'a[T] {
    for i in 0..input.len() {
        if !predicate(&input[i]) {
            println!("{}", i);
            return &input[i..]
        }
    }
    &[]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_skip_while() {
        let slice: &[usize] = &[2, 2, 2, 3, 3, 3];
        assert_eq!(super::skip_while(slice, |&x| x == 2), [3, 3, 3]);
    }
}