#!/usr/bin/env node

const { program } = require('commander');
const fs = require('fs');
const path = require('path');
const rle = require('./src/rle');
const lz = require('./src/lz');

program
    .name('js-compressor')
    .description('Compression tool supporting RLE and LZ77 algorithms')
    .version('1.0.0');

program
    .command('compress <input> <output>')
    .description('Compress a file')
    .option('-r, --rle', 'Use RLE compression')
    .option('-l, --lz', 'Use LZ77 compression')
    .action((input, output, options) => {
        try {
            const inputData = fs.readFileSync(input);
            let compressedData;

            if (options.rle) {
                compressedData = rle.compress(inputData);
            } else if (options.lz) {
                compressedData = lz.compress(inputData);
            } else {
                console.error('Please specify compression algorithm (--rle or --lz)');
                process.exit(1);
            }

            fs.writeFileSync(output, compressedData);
            console.log(`Compression complete. Output written to ${output}`);
        } catch (error) {
            console.error('Error:', error.message);
            process.exit(1);
        }
    });

program
    .command('decompress <input> <output>')
    .description('Decompress a file')
    .option('-r, --rle', 'Use RLE decompression')
    .option('-l, --lz', 'Use LZ77 decompression')
    .action((input, output, options) => {
        try {
            const inputData = fs.readFileSync(input);
            let decompressedData;

            if (options.rle) {
                decompressedData = rle.decompress(inputData);
            } else if (options.lz) {
                decompressedData = lz.decompress(inputData);
            } else {
                console.error('Please specify decompression algorithm (--rle or --lz)');
                process.exit(1);
            }

            fs.writeFileSync(output, decompressedData);
            console.log(`Decompression complete. Output written to ${output}`);
        } catch (error) {
            console.error('Error:', error.message);
            process.exit(1);
        }
    });

program.parse(process.argv); 