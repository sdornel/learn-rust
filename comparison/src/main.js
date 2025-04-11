// FIRST TEST
// basic scraping of URLs
// import got from 'got';
// import fs from 'fs';

// const urls = fs.readFileSync('urls.txt', 'utf-8').trim().split('\n');

// const start = Date.now();

// await Promise.all(
//   urls.map(url => got(url))
// );

// console.log("Node:", (Date.now() - start) / 1000, "seconds");



// SECOND TEST
// includes scraping and word counting
// import { createRequire } from 'module';
// const require = createRequire(import.meta.url);

// const cheerio = require('cheerio');
// import got from 'got';
// import fs from 'fs';

// const urls = fs.readFileSync('urls.txt', 'utf-8').trim().split('\n');

// const start = Date.now();

// await Promise.all(urls.map(async url => {
//   try {
//     const res = await got(url);
//     const $ = cheerio.load(res.body);
//     const text = $('p').text();
//     const wordCount = text.split(/\s+/).length;
//     console.log(`${url.padEnd(60)} -> ${wordCount} words`);
//   } catch (e) {
//     console.error(`Error fetching ${url}: ${e.message}`);
//   }
// }));

// console.log(`Node Done in: ${(Date.now() - start) / 1000}s`);



// THIRD TEST
// print longest 3 summaries per page
import { createRequire } from 'module';
const require = createRequire(import.meta.url);

const cheerio = require('cheerio');
import got from 'got';
import fs from 'fs';

const urls = fs.readFileSync('urls.txt', 'utf-8').trim().split('\n');

function summarize(paragraphs) {
  return paragraphs
    .filter(p => p.length > 50)
    .sort((a, b) => b.length - a.length)
    .slice(0, 3);
}

const start = Date.now();

await Promise.all(urls.map(async (url) => {
  try {
    const res = await got(url);
    const $ = cheerio.load(res.body);
    const paragraphs = $('p').map((_, el) => $(el).text().trim()).get();
    const summary = summarize(paragraphs);

    console.log(`\n🟨 Summary for: ${url}\n`);
    summary.forEach((p, i) => {
      console.log(`${i + 1}. ${p}\n`);
    });
  } catch (err) {
    console.error(`❌ Error: ${url} - ${err.message}`);
  }
}));

console.log(`\nNode Done in: ${(Date.now() - start) / 1000}s`);