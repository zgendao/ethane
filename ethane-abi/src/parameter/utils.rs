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

#[inline]
pub fn right_pad_to_32_bytes(seq: &[u8]) -> [u8; 32] {
    assert!(
        seq.len() <= 32,
        "Input sequence length ({}) must be less or equal to 32",
        seq.len()
    );
    let mut padded = [0u8; 32];
    padded[..seq.len()].copy_from_slice(seq);
    padded
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
            0, 0, 0, 0, 0, 0, 0, 1
        ];
        assert_eq!(expected, left_pad_to_32_bytes(&[1]));

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
        let expected = [
            52, 0, 0, 44, 42, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ];
        assert_eq!(expected, right_pad_to_32_bytes(&[52, 0, 0, 44, 42]));

        let expected = [
            1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0 
        ];
        assert_eq!(expected, right_pad_to_32_bytes(&[1]));

        let expected = [
            23, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 34, 0, 0, 22, 64 
        ];
        assert_eq!(expected, right_pad_to_32_bytes(&expected));
    }
}
