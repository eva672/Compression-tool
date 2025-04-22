const assert = require('assert');
const { compress, decompress } = require('../src/lz');

describe('LZ77 Compression', () => {
    it('should compress and decompress repeated patterns', () => {
        const input = Buffer.from('ABABABABABAB');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });
}); 