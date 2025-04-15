use std::io::{Read, Write};

const WINDOW_SIZE: usize = 20;
const MAX_MATCH_LENGTH: u8 = 255;

pub fn compress<R: Read, W: Write>(mut input: R, mut output: W) -> anyhow::Result<()> {
    let mut window = Vec::with_capacity(WINDOW_SIZE);
    let mut buffer = [0u8; 1];
    let mut pos = 0;

    while let Ok(1) = input.read(&mut buffer) {
        let byte = buffer[0];
        let mut best_match = None;
        let search_start = window.len().saturating_sub(WINDOW_SIZE);

        // Search for the longest match in the sliding window
        for offset in 1..=window.len().saturating_sub(search_start) {
            let mut length = 0;
            while pos + length < window.len()
                && window[pos + length] == window[pos - offset + length]
                && length < MAX_MATCH_LENGTH as usize
            {
                length += 1;
            }

            if length >= 3 && (best_match.is_none() || length > best_match.unwrap().1) {
                best_match = Some((offset as u8, length as u8));
            }
        }

        if let Some((offset, length)) = best_match {
            // Encode match
            output.write_all(&[0x01, offset, length])?;
            pos += length as usize;
        } else {
            // Encode literal
            output.write_all(&[0x00, byte])?;
            window.push(byte);
            pos += 1;
        }
    }

    Ok(())
}

pub fn decompress<R: Read, W: Write>(mut input: R, mut output: W) -> anyhow::Result<()> {
    let mut window = Vec::with_capacity(WINDOW_SIZE);
    let mut buffer = [0u8; 3];

    while let Ok(1) = input.read(&mut buffer[..1]) {
        match buffer[0] {
            0x00 => {
                // Literal
                if let Ok(1) = input.read(&mut buffer[1..2]) {
                    let byte = buffer[1];
                    output.write_all(&[byte])?;
                    window.push(byte);
                }
            }
            0x01 => {
                // Match
                if let Ok(2) = input.read(&mut buffer[1..3]) {
                    let offset = buffer[1] as usize;
                    let length = buffer[2] as usize;
                    let start = window.len().saturating_sub(offset);

                    for i in 0..length {
                        let byte = window[start + i];
                        output.write_all(&[byte])?;
                        window.push(byte);
                    }
                }
            }
            _ => return Err(anyhow::anyhow!("Invalid compression format")),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
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
    fn test_non_repeating_data() {
        let input = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        compress(Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_sliding_window_boundaries() {
        let input = vec![b'A'; 30];
        let mut compressed = Vec::new();
        compress(Cursor::new(&input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed);
    }
}
