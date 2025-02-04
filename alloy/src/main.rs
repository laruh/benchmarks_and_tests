use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolType};

sol! {
    struct MyStruct {
        uint256 a;
        bytes32 b;
        address[] c;
    }
}

fn main() {
    let bytes = vec![];
    let decoded: MyStruct = MyStruct::abi_decode(&bytes, true).unwrap();
    let _rust_a: U256 = decoded.a;
    let _rust_b: [u8; 32] = decoded.b.into();
    let _rust_c: Vec<Address> = decoded.c;
}
