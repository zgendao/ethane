/// Implementation taken shamelessly from
/// [here](https://stackoverflow.com/questions/2423902/convert-an-array-of-bytes-into-one-decimal-number-as-a-string)
pub fn bytes_to_dec_string(bytes: &[u8]) -> String {
    let mut digits = vec![0_u8; (bytes.len() * 0x26882_usize + 0xffff_usize) >> 16];
    let mut length = 1_usize;

    for &byte in bytes.iter() {
        let mut carry = byte as u16;
        let mut i = 0;
        while i < length || carry != 0 {
            let value = digits[i] as u16 * 256_u16 + carry;
            carry = value / 10;
            digits[i] = (value % 10) as u8;
            i += 1;
        }
        if i > length {
            length = i
        }
    }

    let result = digits
        .iter()
        .rev()
        .skip_while(|&digit| digit == &0_u8)
        // unwrap is fine because digit only contains numeric digits
        .map(|&digit| std::char::from_digit(digit.into(), 10).unwrap())
        .collect::<String>();
    // if result = 0, all leading zeros were skipped, we have an empty string
    if result.is_empty() {
        "0".to_owned()
    } else {
        result
    }
}

#[test]
fn dec_string_from_bytes() {
    assert_eq!(&bytes_to_dec_string(Vec::<u8>::new().as_slice()), "0");
    assert_eq!(&bytes_to_dec_string(&[0][..]), "0");
    assert_eq!(&bytes_to_dec_string(&[23][..]), "23");
    assert_eq!(&bytes_to_dec_string(&[0, 212][..]), "212");
    assert_eq!(&bytes_to_dec_string(&[0xfd, 0x32][..]), "64818");
    assert_eq!(
        &bytes_to_dec_string(&[0x23, 0x00, 0xfd, 0x32][..]),
        "587267378"
    );
    assert_eq!(
        &bytes_to_dec_string(
            &[
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
                0xFF, 0x00
            ][..]
        ),
        "22774453838368691933757882222884355840"
    );
    assert_eq!(&bytes_to_dec_string(&[0; 32][..]), "0");
    assert_eq!(
        bytes_to_dec_string(&u128::MAX.to_be_bytes()[..]),
        u128::MAX.to_string()
    );
    // u128::MAX = 340282366920938463463374607431768211455
    let mut u256_bytes = [0_u8; 32];
    u256_bytes[15] = 1; // overflowing u128 by one
    assert_eq!(
        &bytes_to_dec_string(&u256_bytes[..]),
        "340282366920938463463374607431768211456"
    );
}
