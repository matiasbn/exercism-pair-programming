use std::vec;



/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut vec = create_empty();
    for _i in 0..count{
        vec.push(0)
    }
    vec
 }

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
   let mut vec_fibbonacci = create_buffer(5);

   vec_fibbonacci[0] = 1;
   vec_fibbonacci[1] = 1;
  

   for i in 2..vec_fibbonacci.len(){
        vec_fibbonacci[i] = vec_fibbonacci[i - 2] + vec_fibbonacci[i - 1];
        
   }

   vec_fibbonacci

  // either update the vector with numbers manually or create an algo that produces those numbers

   
}
