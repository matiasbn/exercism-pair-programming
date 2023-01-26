pub fn private_key(p: u64) -> u64 {
    for prime in (2..p).rev() {
        for test_value in (2..prime).rev() {
            if prime % test_value == 0 {
                continue;
            }
            if test_value == 2 {
                return prime;
            }
        }
    }
    1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate public key using prime numbers {} and {}, and private key {}",
        p,
        g,
        a
    )
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate secret key using prime number {}, public key {}, and private key {}",
        p,
        b_pub,
        a
    )
}
