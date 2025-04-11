import { createRequire } from 'module';
const require = createRequire(import.meta.url);

import got from 'got';
const cheerio = require('cheerio');
import fs from 'fs';

const urls = fs.readFileSync('urls.txt', 'utf-8').trim().split('\n');

const start = Date.now();

await Promise.all(urls.map(async url => {
  try {
    const res = await got(url);
    const $ = cheerio.load(res.body);
    const text = $('p').text();
    const wordCount = text.split(/\s+/).length;
    console.log(`${url.padEnd(60)} -> ${wordCount} words`);
  } catch (e) {
    console.error(`Error fetching ${url}: ${e.message}`);
  }
}));

console.log(`Node Done in: ${(Date.now() - start) / 1000}s`);