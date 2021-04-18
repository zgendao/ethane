/// Left pads a slice of bytes with preceding zeros such that the
/// result is a 32 byte fixed array.
///
/// # Panics
/// Panics if the input sequence lenght is greater than 32.
#[inline]
pub fn left_pad_to_32_bytes(seq: &[u8]) -> [u8; 32] {
    assert!(
        seq.len() <= 32,
        "Input sequence length ({}) must be less or equal to 32",
        seq.len()
    );
    let mut padded = [0u8; 32];
    let diff = 32 - seq.len();
    padded[diff..].copy_from_slice(seq);
    padded
}

/// Right pads a dynamic slice of bytes with trailing zeros such that the
/// resulting vector will have a length which is a multiple of 32.
#[inline]
#[rustfmt::skip]
pub fn right_pad_to_32_multiples(seq: &[u8]) -> Vec<u8> {
    let len = seq.len();
    let mut i = 1;
    while i * 32 < len { i += 1 }
    let mut bytes = vec![0u8; i * 32];

    bytes[..len].copy_from_slice(seq);
    bytes.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn left_pad() {
        let expected = [
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 17
        ];
        assert_eq!(expected, left_pad_to_32_bytes(&[17]));

        let expected = [
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 34, 0, 0, 22, 64 
        ];
        assert_eq!(expected, left_pad_to_32_bytes(&[34, 0, 0, 22, 64]));

        let expected = [
            23, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 34, 0, 0, 22, 64 
        ];
        assert_eq!(expected, left_pad_to_32_bytes(&expected));
    }

    #[test]
    #[rustfmt::skip]
    fn right_pad() {
        let expected = vec![
            52, 0, 0, 44, 42, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ];
        assert_eq!(expected, right_pad_to_32_multiples(&[52, 0, 0, 44, 42]));

        let expected = vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 2, 
            0, 0, 3, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 
        ];
        assert_eq!(expected, right_pad_to_32_multiples(&[
            1, 2, 3, 4, 5, 6, 7, 8,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 2, 
            0, 0, 3
        ]));

        let expected = vec![
            23, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 34, 0, 0, 22, 64 
        ];
        assert_eq!(expected, right_pad_to_32_multiples(&expected));

        let expected = vec![
            23, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 34, 0, 0, 22, 64,
            23, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 34, 0, 0, 22, 64 
        ];
        assert_eq!(expected, right_pad_to_32_multiples(&expected));
    }
}
