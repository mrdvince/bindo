fn main() {}

#[cfg(test)]
mod tests {
    use bindo::{compress, decompress};

    #[test]
    fn compress_and_decompress_works() {
        let input = include_str!("../random.txt").as_bytes();
        // let input = "hello world".as_bytes();

        let compressed_out = compress(input);
        let decompressed_out = decompress(compressed_out, input.len());
        println!("{:?}, \n {:?}", input, decompressed_out);
        assert_eq!(input, &decompressed_out[..]);
    }
}
