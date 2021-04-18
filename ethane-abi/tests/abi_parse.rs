use ethane_abi::{Abi, Parameter};
use ethereum_types::{Address, U256};
use hex_literal::hex;

use std::path::Path;
use std::str::FromStr;

#[test]
#[rustfmt::skip]
fn test_abi_encode() {
    let path = Path::new("tests/foo.abi");
    let mut abi = Abi::new();
    abi.parse_file(path).expect("unable to parse abi");

	// first encode attempt
    let address = Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
    let hash = abi.encode("bar", vec![Parameter::from(address)]);
    let expected = hex!("
    	646ea56d
    	00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a"
    );
    assert_eq!(hash.unwrap(), expected);

    // second encode attempt
    let address = Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
    let hash = abi.encode("approve", vec![
    	Parameter::from(address),
    	Parameter::from(U256::from_str("613").unwrap())
    ]);
    let expected = hex!("
        095ea7b3
        00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a
        0000000000000000000000000000000000000000000000000000000000000613"
    );
    assert_eq!(hash.unwrap(), expected);

	// third encode attempt
    let hash = abi.encode("totalSupply", vec![]);
    let expected = hex!("18160DDD");
    assert_eq!(hash.unwrap(), expected);

	let address1 = Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
	let address2 = Address::from_str("0x1A4C0439ba035DAcf0D573394107597CEEBF9FF8").unwrap();
    let hash = abi.encode("transferFrom", vec![
            Parameter::from(address1),
            Parameter::from(address2),
            Parameter::from(U256::from_str("14DDD").unwrap()),
        ]);
    let expected =hex!("
        23b872dd
        00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a
        0000000000000000000000001a4c0439ba035dacf0d573394107597ceebf9ff8
        0000000000000000000000000000000000000000000000000000000000014ddd"
    );
    assert_eq!(hash.unwrap(), expected);
}