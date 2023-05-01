/// Constructs a byte array of const length and populates it with the Unicode byte sequence
/// represented by the passed &str, padded with zeroes.
pub const fn fixed_length_str<const L: usize>(val: &str) -> [u8; L] {
    assert!(val.len() <= L);
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
