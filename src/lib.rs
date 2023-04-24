const MASK: u8 = 1 << 5;

pub fn is_uppercase(c: char) -> bool {
    c as u8 & MASK == 0
}

pub fn is_lowercase(c: char) -> bool {
    c as u8 & MASK != 0
}

pub fn toggle_case(c: char) -> char {
    (c as u8 ^ MASK) as _
}

pub fn to_uppercase(c: char) -> char {
    if is_uppercase(c) {
        c
    } else {
        toggle_case(c)
    }
}

pub fn to_lowercase(c: char) -> char {
    if is_lowercase(c) {
        c
    } else {
        toggle_case(c)
    }
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
