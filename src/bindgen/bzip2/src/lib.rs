
use std::io::{self, Read, Write};
use std::mem;


/// Automatically generated bindings to `libbzip2`, DO NOT EDIT.
pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


pub struct Compressor {
    stream: Box<ffi::bz_stream>,
}

impl Compressor {
    pub fn new() -> Result<Compressor, Bzip2Error> {
        unsafe {
            let mut comp = Compressor { stream: Box::new(mem::zeroed()) };
            let result = ffi::BZ2_bzCompressInit(&mut *comp.stream,
                                                 1, // 1 x 100000 block size
                                                 0, // verbosity (4 = most verbose)
                                                 0); // default work factor
            match result as u32 {
                ffi::BZ_OK => Ok(comp),
                other => Err((other as i32).into()),
            }
        }
    }

    pub fn compress<R: Read, W: Write>(&mut self,
                                       mut src: R,
                                       mut dest: W)
                                       -> Result<(), Bzip2Error> {
        let mut input = vec![];
        src.read_to_end(&mut input)?;
        let mut compressed_output = vec![0; input.len()];

        self.stream.next_in = input.as_ptr() as *mut _;
        self.stream.avail_in = input.len() as _;
        self.stream.next_out = compressed_output.as_mut_ptr() as *mut _;
        self.stream.avail_out = compressed_output.len() as _;

        unsafe {
            let result = ffi::BZ2_bzCompress(&mut *self.stream, ffi::BZ_FINISH as _);
            match result as u32 {
                ffi::BZ_FINISH_OK |
                ffi::BZ_RUN_OK |
                ffi::BZ_FLUSH_OK |
                ffi::BZ_STREAM_END => {
                    dest.write_all(&mut compressed_output)
                        .map(|_| ())
                        .map_err(|e| e.into())
                }
                other => Err((other as i32).into()),
            }
        }
    }
}


impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe {
            ffi::BZ2_bzCompressEnd(&mut *self.stream);
        }
    }
}


#[derive(Debug)]
pub enum Bzip2Error {
    Config,
    Params,
    Memory,
    InvalidSequence,
    Io(Box<io::Error>),
}

impl From<i32> for Bzip2Error {
    fn from(val: i32) -> Bzip2Error {
        match val {
            ffi::BZ_CONFIG_ERROR => Bzip2Error::Config,
            ffi::BZ_PARAM_ERROR => Bzip2Error::Params,
            ffi::BZ_MEM_ERROR => Bzip2Error::Memory,
            ffi::BZ_SEQUENCE_ERROR => Bzip2Error::InvalidSequence,
            unknown => panic!("Invalid error code: {}", unknown),
        }
    }
}

impl From<io::Error> for Bzip2Error {
    fn from(val: io::Error) -> Bzip2Error {
        Bzip2Error::Io(Box::new(val))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn create_compressor() {
        let dest: Vec<u8> = vec![];
        let got = Compressor::new();
        assert!(got.is_ok());
    }

    #[test]
    fn try_compress() {
        let src = "Hello World".to_string();
        let mut dest = vec![];
        let mut compressor = Compressor::new().unwrap();

        let got = compressor.compress(Cursor::new(src), &mut dest);
        println!("{:?}", dest);
        println!("{:?}", got);
        assert!(got.is_ok());
    }




    #[test]
    fn round_trip_compression_decompression() {
        use ffi::*;
        unsafe {
            let input = include_str!("../Cargo.toml").as_bytes();
            let mut compressed_output: Vec<u8> = vec![0; input.len()];
            let mut decompressed_output: Vec<u8> = vec![0; input.len()];

            // Construct a compression stream.
            let mut stream: bz_stream = mem::zeroed();
            let result = BZ2_bzCompressInit(&mut stream as *mut _,
                                            1, // 1 x 100000 block size
                                            0, // verbosity (4 = most verbose)
                                            0); // default work factor
            match result {
                r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"),
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
                r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),
                r if r == (BZ_OK as _) => {}
                r => panic!("Unknown return value = {}", r),
            }

            // Compress `input` into `compressed_output`.
            stream.next_in = input.as_ptr() as *mut _;
            stream.avail_in = input.len() as _;
            stream.next_out = compressed_output.as_mut_ptr() as *mut _;
            stream.avail_out = compressed_output.len() as _;
            let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
            match result {
                r if r == (BZ_RUN_OK as _) => panic!("BZ_RUN_OK"),
                r if r == (BZ_FLUSH_OK as _) => panic!("BZ_FLUSH_OK"),
                r if r == (BZ_FINISH_OK as _) => panic!("BZ_FINISH_OK"),
                r if r == (BZ_SEQUENCE_ERROR as _) => panic!("BZ_SEQUENCE_ERROR"),
                r if r == (BZ_STREAM_END as _) => {}
                r => panic!("Unknown return value = {}", r),
            }

            // Finish the compression stream.
            let result = BZ2_bzCompressEnd(&mut stream as *mut _);
            match result {
                r if r == (BZ_PARAM_ERROR as _) => panic!(BZ_PARAM_ERROR),
                r if r == (BZ_OK as _) => {}
                r => panic!("Unknown return value = {}", r),
            }

            // Construct a decompression stream.
            let mut stream: bz_stream = mem::zeroed();
            let result = BZ2_bzDecompressInit(&mut stream as *mut _,
                                              4, // verbosity (4 = most verbose)
                                              0); // default small factor
            match result {
                r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"),
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
                r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),
                r if r == (BZ_OK as _) => {}
                r => panic!("Unknown return value = {}", r),
            }

            // Decompress `compressed_output` into `decompressed_output`.
            stream.next_in = compressed_output.as_ptr() as *mut _;
            stream.avail_in = compressed_output.len() as _;
            stream.next_out = decompressed_output.as_mut_ptr() as *mut _;
            stream.avail_out = decompressed_output.len() as _;
            let result = BZ2_bzDecompress(&mut stream as *mut _);
            match result {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
                r if r == (BZ_DATA_ERROR as _) => panic!("BZ_DATA_ERROR"),
                r if r == (BZ_DATA_ERROR_MAGIC as _) => panic!("BZ_DATA_ERROR"),
                r if r == (BZ_MEM_ERROR as _) => panic!("BZ_MEM_ERROR"),
                r if r == (BZ_OK as _) => panic!("BZ_OK"),
                r if r == (BZ_STREAM_END as _) => {}
                r => panic!("Unknown return value = {}", r),
            }

            // Close the decompression stream.
            let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
            match result {
                r if r == (BZ_PARAM_ERROR as _) => panic!("BZ_PARAM_ERROR"),
                r if r == (BZ_OK as _) => {}
                r => panic!("Unknown return value = {}", r),
            }

            assert_eq!(input, &decompressed_output[..]);
        }
    }
}
