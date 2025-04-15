const assert = require('assert');
const { compress, decompress } = require('../src/lz');

describe('LZ77 Compression', () => {
    it('should compress and decompress repeated patterns', () => {
        const input = Buffer.from('ABABABABABAB');
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

    it('should handle non-repeating data', () => {
        const input = Buffer.from('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle sliding window boundaries', () => {
        const input = Buffer.from('A'.repeat(30) + 'B'.repeat(30));
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });
}); 