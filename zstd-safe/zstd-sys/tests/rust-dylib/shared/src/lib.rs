pub fn encode(input: &[u8]) -> Vec<u8> {
    zstd::bulk::compress(input, 1).unwrap()
}
