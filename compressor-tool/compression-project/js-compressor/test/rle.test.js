const assert = require('assert');
const { compress, decompress } = require('../src/rle');

describe('RLE Compression', () => {
    it('should compress and decompress simple repeated data', () => {
        const input = Buffer.from('AAABBBCCCCCDDDDE');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });
}); 