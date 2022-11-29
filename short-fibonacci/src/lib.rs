/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fibonacci = create_buffer(5);
    for i in 0..fibonacci.len() {
        fibonacci[i] += if i == 0 || i == 1 {
            1
        } else {
            fibonacci[i - 1] + fibonacci[i - 2]
        };
    }
    fibonacci
}

// #[test]
// fn test_fibonacci() {
//     let fibo = fibonacci();
//     println!("{:?}", fibo);
// }
