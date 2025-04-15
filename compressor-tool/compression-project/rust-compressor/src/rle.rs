use std::io::{Read, Write};

pub fn compress<R: Read, W: Write>(mut input: R, mut output: W) -> anyhow::Result<()> {
    let mut buffer = [0u8; 1];
    let mut current_byte = None;
    let mut count = 0u8;

    while let Ok(1) = input.read(&mut buffer) {
        let byte = buffer[0];

        match current_byte {
            Some(b) if b == byte && count < 255 => {
                count += 1;
            }
            Some(b) => {
                output.write_all(&[count, b])?;
                current_byte = Some(byte);
                count = 1;
            }
            None => {
                current_byte = Some(byte);
                count = 1;
            }
        }
    }

    if let Some(b) = current_byte {
        output.write_all(&[count, b])?;
    }

    Ok(())
}

pub fn decompress<R: Read, W: Write>(mut input: R, mut output: W) -> anyhow::Result<()> {
    let mut buffer = [0u8; 2];

    while let Ok(2) = input.read(&mut buffer) {
        let count = buffer[0];
        let byte = buffer[1];

        for _ in 0..count {
            output.write_all(&[byte])?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let mut compressed = Vec::new();
        compress(Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_empty_input() {
        let input = b"";
        let mut compressed = Vec::new();
        compress(Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_single_byte() {
        let input = b"A";
        let mut compressed = Vec::new();
        compress(Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_max_run_length() {
        let input = vec![b'A'; 255];
        let mut compressed = Vec::new();
        compress(Cursor::new(&input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed);
    }
}
