fn main() {}

#[cfg(test)]
mod tests {
    use bindo::{compress, decompress};

    #[test]
    fn compress_and_decompress_works() {
        let input = include_str!("../random.txt").as_bytes();
        let compressed_output = compress(input);
        let decompressed_output = decompress(compressed_output, input.len());
        assert_eq!(input, &decompressed_output[..]);
    }

    // #[test]
    // #[should_panic(expected = "BZ_PARAM_ERROR")]
    // fn compress_handles_bad_parameter() {
    //     let input = vec![0; 100];
    //     let compressed = compress(&input[..]);
    // }

    // #[test]
    // #[should_panic(expected = "BZ_MEM_ERROR")]
    // fn compress_handles_memory_error() {
    //     let input = vec![0; 1000000000];
    //     let compressed = compress(&input[..]);
    // }

    // #[test]
    // #[should_panic(expected = "BZ_CONFIG_ERROR")]
    // fn decompress_handles_config_error() {
    //     let compressed = vec![0; 100];
    //     let decompressed = decompress(&compressed[..], 1000);
    // }

    // #[test]
    // #[should_panic(expected = "BZ_PARAM_ERROR")]
    // fn decompress_handles_bad_parameter() {
    //     let compressed = vec![0; 100];
    //     let decompressed = decompress(&compressed[..], 0);
    // }

    // #[test]
    // #[should_panic(expected = "BZ_MEM_ERROR")]
    // fn decompress_handles_memory_error() {
    //     let compressed = vec![0; 1000000000];
    //     let decompressed = decompress(&compressed[..], 1000);
    // }
}
