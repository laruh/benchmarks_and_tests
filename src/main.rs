use benchmarks_and_tests::{extend_from_slice, extend_with_chain};
use std::time::Instant;
use url::Url;

const MORALIS_API: &str = "api";
const MORALIS_ENDPOINT_V: &str = "v2";
const MORALIS_FORMAT_QUERY_NAME: &str = "format";
const MORALIS_FORMAT_QUERY_VALUE: &str = "decimal";

pub fn main() {
    let funding_timelock_bytes = vec![1; 1000];
    let payment_timelock_bytes = vec![2; 1000];
    let secret_hash = vec![3; 1000];

    let iterations = 10000;

    // Test extend_from_slice
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = extend_from_slice(
            &funding_timelock_bytes,
            &payment_timelock_bytes,
            &secret_hash,
        );
    }
    let duration_extend_from_slice = start.elapsed();

    println!(
        "Time taken for extend_from_slice: {:?}",
        duration_extend_from_slice
    );

    // Test extend_with_chain
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = extend_with_chain(
            &funding_timelock_bytes,
            &payment_timelock_bytes,
            &secret_hash,
        );
    }
    let duration_extend_with_chain = start.elapsed();

    println!(
        "Time taken for extend_with_chain: {:?}",
        duration_extend_with_chain
    );
}

#[allow(dead_code)]
fn construct_moralis_uri_for_nft(base_url: &mut Url, address: &str, chain: &str) -> Url {
    let mut uri = base_url.clone();
    uri.path_segments_mut()
        .unwrap()
        .push(MORALIS_API)
        .push(MORALIS_ENDPOINT_V)
        .push(address)
        .push("nft");
    uri.query_pairs_mut()
        .append_pair("chain", chain)
        .append_pair(MORALIS_FORMAT_QUERY_NAME, MORALIS_FORMAT_QUERY_VALUE);
    uri
}

/*
use bigdecimal::BigDecimal;
use std::str::FromStr;
use ethereum_types::Address;
use hyper::{Body, Request, Uri};
use ethabi::Token;

    // Code to test Uri building
    let taker_address = Address::from_str("0x16e281a9f2E7581269A13E516Aa79d6A4A1Cd980").unwrap();
    println!("taker_address {:?}", Token::Address(taker_address));
    let token_address = Address::from_str("0xbac1c9f2087f39caaa4e93412c6412809186870e").unwrap();
    println!("token_address {:?}", Token::Address(token_address));
    let encoded = ethabi::encode(&[Token::Address(taker_address),Token::Address(token_address)]);
    println!("encoded {:?}", encoded);

    let from_float: BigDecimal = 0.95.try_into().unwrap();
    println!("BigDecimal from float literal: {}", from_float);

    let from_str = BigDecimal::from_str("0.95").expect("Failed to parse threshold value");
    println!("BigDecimal from string: {}", from_str);

    let uri: Uri = "https://deep-index.moralis.io/api/v2.2/0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326/nft/transfers?chain=eth&format=decimal&order=DESC".parse().unwrap();
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    println!("uri: {}", req.uri());
    println!("uri: {}", req.uri().path());

    let mut base_url = Url::parse("https://example.com/nft-route").unwrap();
    let address = "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326";

    let result = construct_moralis_uri_for_nft(&mut base_url, address, "eth");
    println!("{}", result);
 */
