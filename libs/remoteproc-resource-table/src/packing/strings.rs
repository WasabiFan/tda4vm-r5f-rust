/// Constructs a byte array of const length and populates it with the Unicode byte sequence
/// represented by the passed &str, padded with zeroes.
///
/// Requires val.len() < L, to allow for at least one null terminator byte.
pub const fn fixed_length_str<const L: usize>(val: &str) -> [u8; L] {
    assert!(val.len() < L);
    let mut result = [0; L];

    let mut i = 0;
    loop {
        if i >= val.len() {
            break;
        }
        result[i] = val.as_bytes()[i];
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::fixed_length_str;

    #[test]
    fn empty_string_min_length() {
        // Given
        let s = "";

        // When
        let b = fixed_length_str::<1>(s);

        // Then
        assert_eq!(b, [0]);
    }

    #[test]
    fn empty_string() {
        // Given
        let s = "";

        // When
        let b = fixed_length_str::<10>(s);

        // Then
        assert_eq!(b, [0; 10]);
    }

    #[test]
    fn max_length_string() {
        // Given
        let s = "123456789";

        // When
        let b = fixed_length_str::<10>(s);

        // Then
        assert_eq!(b, [0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0]);
    }

    #[test]
    #[should_panic = ""]
    fn string_length_equal_array_length_panics() {
        // Given
        let s = "1234567890";

        // When
        let _b = fixed_length_str::<10>(s);

        // Then
        // Panic
    }

    #[test]
    #[should_panic]
    fn zero_array_length_should_panic() {
        // Given
        let s = "";

        // When
        let _b = fixed_length_str::<0>(s);

        // Then
        // Panic
    }
}
