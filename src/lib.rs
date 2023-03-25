#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn compress(input: &[u8], size_input: usize) -> Vec<u8> {
    unsafe {
        
        let mut compressed_out: Vec<u8> = vec![0; size_input];
        // Compression stream
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzCompressInit(
            &mut stream as *mut _,
            1, // 1 x 100000 block size
            4, // verbosity (4 = most verbose)
            0,
        ); // default work factor
        match result {
            r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"),
            r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
            r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),
            r if r == (BZ_OK as _) => {}
            r => panic!("Unknown return value = {}", r),
        }
        // Compress `input` into `compressed_out`.
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = compressed_out.as_mut_ptr() as *mut _;
        stream.avail_out = compressed_out.len() as _;
        let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        // match result {
        //     r if r == (BZ_RUN_OK as _) => panic!("BZ_RUN_OK"),
        //     r if r == (BZ_FLUSH_OK as _) => panic!("BZ_FLUSH_OK"),
        //     r if r == (BZ_FINISH_OK as _) => panic!("BZ_FINISH_OK"),
        //     r if r == (BZ_SEQUENCE_ERROR as _) => panic!("BZ_SEQUENCE_ERROR"),
        //     r if r == (BZ_STREAM_END as _) => {}
        //     r => panic!("Unknown return value = {}", r),
        // }
        // Finish the compression stream.
        let result = BZ2_bzCompressEnd(&mut stream as *mut _);
        match result {
            r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
            r if r == (BZ_OK as _) => {}
            r => panic!("Unknown return value = {}", r),
        }
        compressed_out
    }
}

pub fn decompress(compressed: &[u8], size_input: usize) -> Vec<u8> {
    unsafe {
        let mut decompressed_out: Vec<u8> = vec![0; size_input];

        // Construct a decompression stream.
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzDecompressInit(
            &mut stream as *mut _,
            4, // verbosity (4 = most verbose)
            0,
        ); // default small factor
        match result {
            r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"),
            r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
            r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),
            r if r == (BZ_OK as _) => {}
            r => panic!("Unknown return value = {}", r),
        }

        // Decompress `compressed_output` into `decompressed_out`.
        stream.next_in = compressed.as_ptr() as *mut _;
        stream.avail_in = compressed.len() as _;
        stream.next_out = decompressed_out.as_mut_ptr() as *mut _;
        stream.avail_out = decompressed_out.len() as _;
        let result = BZ2_bzDecompress(&mut stream as *mut _);
        // match result {
        //     r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
        //     r if r == (BZ_DATA_ERROR as _) => panic!("BZ_DATA_ERROR"),
        //     r if r == (BZ_DATA_ERROR_MAGIC as _) => panic!("BZ_DATA_ERROR"),
        //     r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),
        //     r if r == (BZ_OK as _) => panic!("BZ_OK"),
        //     r if r == (BZ_STREAM_END as _) => {}
        //     r => panic!("Unknown return value = {}", r),
        // }

        // Close the decompression stream.
        let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
        match result {
            r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
            r if r == (BZ_OK as _) => {}
            r => panic!("Unknown return value = {}", r),
        }
        decompressed_out
    }
}
