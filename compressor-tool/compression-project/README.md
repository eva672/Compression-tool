# Compression Tool

A command-line tool for file compression and decompression, implemented in both Rust and JavaScript. Supports two compression algorithms: Run-Length Encoding (RLE) and Simplified LZ77.

## Features

- Compress and decompress files using RLE or LZ77 algorithms
- Available as Docker images
- Cross-platform support
- Comprehensive test coverage
- Performance benchmarking

## Installation

### Using Docker

```bash
# Pull the Docker images
docker pull ghcr.io/your-org-name/rust-compressor:latest
docker pull ghcr.io/your-org-name/js-compressor:latest
```

### Building from Source

#### Rust Implementation

```bash
cd rust-compressor
cargo build --release
```

#### JavaScript Implementation

```bash
cd js-compressor
npm install
```

## Usage

### Rust Implementation

```bash
# Compress a file using RLE
./rust-compressor compress input.txt output.txt --rle

# Compress a file using LZ77
./rust-compressor compress input.txt output.txt --lz

# Decompress a file using RLE
./rust-compressor decompress input.txt output.txt --rle

# Decompress a file using LZ77
./rust-compressor decompress input.txt output.txt --lz
```

### JavaScript Implementation

```bash
# Compress a file using RLE
node index.js compress input.txt output.txt --rle

# Compress a file using LZ77
node index.js compress input.txt output.txt --lz

# Decompress a file using RLE
node index.js decompress input.txt output.txt --rle

# Decompress a file using LZ77
node index.js decompress input.txt output.txt --lz
```

### Using Docker

```bash
# Compress using Rust implementation
docker run -v $(pwd):/data ghcr.io/your-org-name/rust-compressor compress /data/input.txt /data/output.txt --rle

# Compress using JavaScript implementation
docker run -v $(pwd):/data ghcr.io/your-org-name/js-compressor compress /data/input.txt /data/output.txt --rle
```

## Compression Algorithms

### Run-Length Encoding (RLE)

RLE is a simple compression algorithm that replaces consecutive identical bytes with a count and the byte value. It works best with data containing many repeated bytes.

### Simplified LZ77

LZ77 is a dictionary-based compression algorithm that replaces repeated sequences with references to previous occurrences. This implementation uses a sliding window of 20 bytes and simplified encoding:

- Literal: `0x00 + byte`
- Match: `0x01 + offset + length`

## Testing

### Rust

```bash
cd rust-compressor
cargo test
```

### JavaScript

```bash
cd js-compressor
npm test
```

## Performance Benchmarking

To compare the performance of different implementations and algorithms:

```bash
# Benchmark Rust implementation
time docker run -v $(pwd):/data ghcr.io/your-org-name/rust-compressor compress /data/test.txt /data/test.txt.cmp --rle

# Benchmark JavaScript implementation
time docker run -v $(pwd):/data ghcr.io/your-org-name/js-compressor compress /data/test.txt /data/test.txt.cmp --rle
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
