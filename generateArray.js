import { createReadStream } from "fs"
import { writeFile } from "node:fs/promises"

const readStream = createReadStream('./dictionary.txt', { highWaterMark: 16 });
const data = [];

const validWords = []

readStream.on('data', (chunk) => {
    data.push(chunk);
});

readStream.on('end', async () => {
    const words = Buffer.concat(data).toString().replace("\r", "").split("\n")
    words.forEach((word) => {
        word = word.trim()
        validWords.push(word)
    })

    await writeFile("dictionary.jsonf", JSON.stringify(validWords))
})

readStream.on('error', (err) => {
    console.log('error :', err)
})