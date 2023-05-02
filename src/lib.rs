#[inline]
pub fn is_uppercase(c: char) -> bool {
    c as u8 & 1 << 5 == 0
}

#[inline]
pub fn is_lowercase(c: char) -> bool {
    c as u8 & 1 << 5 != 0
}

#[inline]
pub fn toggle_case(c: char) -> char {
    (c as u8 ^ 1 << 5) as _
}

#[inline]
pub fn to_uppercase(c: char) -> char {
    (c as u8 ^ (is_lowercase(c) as u8 * (1 << 5))) as _
}

#[inline]
pub fn to_lowercase(c: char) -> char {
    (c as u8 | (is_uppercase(c) as u8 * (1 << 5))) as _
}

#[cfg(test)]
mod tests {
    #[test]
    fn toggle_case() {
        assert_eq!(super::toggle_case('a'), 'A');
        assert_eq!(super::toggle_case('A'), 'a');
    }

    #[test]
    fn uppercase() {
        assert_eq!(super::to_uppercase('a'), 'A');
        assert_eq!(super::to_uppercase('A'), 'A');

        assert!(super::is_uppercase('A'));
    }

    #[test]
    fn lowercase() {
        assert_eq!(super::to_lowercase('A'), 'a');
        assert_eq!(super::to_lowercase('a'), 'a');

        assert!(super::is_lowercase('a'));
    }
}
