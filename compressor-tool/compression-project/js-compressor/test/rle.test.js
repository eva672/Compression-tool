const assert = require('assert');
const { compress, decompress } = require('../src/rle');

describe('RLE Compression', () => {
    it('should compress and decompress simple repeated data', () => {
        const input = Buffer.from('AAABBBCCCCCDDDDE');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle empty input', () => {
        const input = Buffer.alloc(0);
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.length, 0);
    });

    it('should handle single byte input', () => {
        const input = Buffer.from('A');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle maximum run length (255)', () => {
        const input = Buffer.alloc(255, 'A');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });
}); 