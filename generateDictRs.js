import { createReadStream } from "fs"
import { writeFile } from "node:fs/promises"

const readStream = createReadStream('./dictionary.jsonf', { highWaterMark: 16 });
const data = [];

readStream.on('data', (chunk) => {
    data.push(chunk);
});

readStream.on('end', async () => {
    const words = JSON.parse(Buffer.concat(data).toString())

    await writeFile("./crates/dictionary/src/dict.rs", `pub static DICTIONARY:[&str;${words.length}]=${JSON.stringify(words)};`)
})

readStream.on('error', (err) => {
    console.log('error :', err)
})