use std::time::Instant;

struct BlockHeader<'a> {
    version: &'a str,
    previous_block: &'a str,
    merkle_root: &'a str,
    timestamp: &'a str,
    nonce: &'a str,
}

impl<'a> BlockHeader<'a> {
    fn parse(input: &'a str) -> BlockHeader<'a> {
        if !input.contains('|') {
            panic!("MALFORMED_HEADER: Input does not contain the '|' delimiter.");
        }
        let mut version = "";
        let mut previous_block = "";
        let mut merkle_root = "";
        let mut timestamp = "";
        let mut nonce = "";

        for segment in input.split('|') {
            if let Some((key, value)) = segment.split_once(':') {
                match key {
                    "version" => version = value,
                    "prev_block" => previous_block = value,
                    "merkle_root" => merkle_root = value,
                    "timestamp" => timestamp = value,
                    "nonce" => nonce = value,
                    _ => {}
                }
            }
        }
        BlockHeader {
            version,
            previous_block,
            merkle_root,
            timestamp,
            nonce,
        }
    }
}

fn main() {
    // 1. Create a raw string to simulate high-load infra
    let raw_data = "version:2.0|prev_block:0x0000000000000abc123|merkle_root:0xdef456|timestamp:1714321713|nonce:999999999";

    // 2. Start the Clock
    let start = Instant::now();

    // 3. The Zero-Copy Parse
    let header = BlockHeader::parse(raw_data);

    // 4. Stop the Clock
    let duration = start.elapsed();

    println!("--- Stats ---");
    println!("Time taken: {:?}", duration);
    println!("Input size: {} bytes", raw_data.len());
    println!(
        "Struct size on stack: {} bytes",
        std::mem::size_of_val(&header)
    );
    println!("-------------------------------");

    // Display the results
    println!("Parsed Nonce: {}", header.nonce);
    println!("Parsed Version: {}", header.version);
    println!("Parsed Previous Block: {}", header.previous_block);
    println!("Parsed Merkle Root: {}", header.merkle_root);
    println!("Parsed Timestamp: {}", header.timestamp);
}

#[cfg(test)] // Dabbling in tests, i feel like i left it too late. will add as extensive of a test suite as i can figure out from now on
mod tests {
    use super::*; // Allows the tests to "see" your BlockHeader and parse function

    #[test]

    //simple happy path test
    fn test_field_mapping() {
        let input = "version:1|nonce:999";
        let header = BlockHeader::parse(input);

        assert_eq!(header.version, "1");
        assert_eq!(header.nonce, "999");
    }

    #[test]

    fn test_malformed_input() {
        let input = "broken_segment|nonce:123";
        let header = BlockHeader::parse(input);

        // Ensure valid data was still caught despite the break
        assert_eq!(header.nonce, "123");
    }

    // unhappy path, sad path ? idk.
    // all i know is this should panic because there are no '|' delimiters in the simulated input, which is required for the parser to function correctly.
    #[test]
    #[should_panic(expected = "MALFORMED_HEADER")]
    fn test_no_pipes_should_panic() {
        let input = "version1nonce999";
        BlockHeader::parse(input);
    }
}
