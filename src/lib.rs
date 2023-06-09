#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub fn compress(input: &[u8]) -> Vec<u8> {
    unsafe {
        let mut compressed_out: Vec<u8> = vec![0; input.len()];

        // Construct a compression stream.
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzCompressInit(
            &mut stream as *mut _,
            1, // 1 x 100000 block size
            4, // verbosity (4 = most verbose)
            0,
        ); // default work factor
        handle_bz_error(result, "BZ2_bzCompressInit");

        // Compress `input` into `compressed_out`.
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = compressed_out.as_mut_ptr() as *mut _;
        stream.avail_out = compressed_out.len() as _;
        let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        handle_bz_error(result, "BZ2_bzCompress");

        // Finish the compression stream.
        let result = BZ2_bzCompressEnd(&mut stream as *mut _);
        handle_bz_error(result, "BZ2_bzCompressEnd");
        compressed_out
    }
}

pub fn decompress(compressed_out: Vec<u8>, comp_len: usize) -> Vec<u8> {
    unsafe {
        let mut decompressed_out: Vec<u8> = vec![0; comp_len];
        // Construct a decompression stream.
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzDecompressInit(
            &mut stream as *mut _,
            4, // verbosity (4 = most verbose)
            0,
        ); // default small factor
        handle_bz_error(result, "BZ2_bzDecompressInit");

        // Decompress `compressed_out` into `decompressed_out`.
        stream.next_in = compressed_out.as_ptr() as *mut _;
        stream.avail_in = compressed_out.len() as _;
        stream.next_out = decompressed_out.as_mut_ptr() as *mut _;
        stream.avail_out = decompressed_out.len() as _;
        let result = BZ2_bzDecompress(&mut stream as *mut _);
        handle_bz_error(result, "BZ2_bzDecompress");
        
        // Close the decompression stream.
        let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
        handle_bz_error(result, "BZ2_bzDecompressEnd");
        decompressed_out
    }
}

fn handle_bz_error(result: i32, name: &str) {
    match result {
        r if r == (BZ_CONFIG_ERROR as _) => panic!("{}: BZ_CONFIG_ERROR", name),
        r if r == (BZ_PARAM_ERROR as _) => panic!("{}: BZ_PARAM_ERROR", name),
        r if r == (BZ_MEM_ERROR as _) => panic!("{}: BZ_MEM_ERROR", name),
        r if r == (BZ_OK as _) => {}
        r if r == (BZ_RUN_OK as _) => panic!("BZ_RUN_OK"),
        r if r == (BZ_FLUSH_OK as _) => panic!("BZ_FLUSH_OK"),
        r if r == (BZ_FINISH_OK as _) => panic!("BZ_FINISH_OK"),
        r if r == (BZ_SEQUENCE_ERROR as _) => panic!("BZ_SEQUENCE_ERROR"),
        r if r == (BZ_STREAM_END as _) => {}
        r if r == (BZ_DATA_ERROR as _) => panic!("BZ_DATA_ERROR"),
        r if r == (BZ_DATA_ERROR_MAGIC as _) => panic!("BZ_DATA_ERROR"),
        r => panic!("{}: Unknown return value = {}", name, r),
    }
}
