fn main() {
    let input = b"shared library round trip";
    let compressed = zstd_dylib_shared::encode(input);
    // Instantiate the generic stream decoder in the consumer, outside the dylib.
    let decoded = zstd::stream::decode_all(compressed.as_slice()).unwrap();
    assert_eq!(decoded, input);

    #[cfg(feature = "seekable")]
    unsafe {
        extern "C" {
            fn ZSTD_seekable_create() -> *mut std::ffi::c_void;
            fn ZSTD_seekable_free(context: *mut std::ffi::c_void) -> usize;
        }
        let context = ZSTD_seekable_create();
        assert!(!context.is_null());
        assert_eq!(ZSTD_seekable_free(context), 0);
    }
}
