import fs from "fs";

const file = process.argv[1];

// Copy built file
fs.mkdirSync("temp-target");
fs.copyFileSync(
  `target/wasm32-wasi/debug/${file}.wasm`,
  `temp-target/${file}.wasm`
);

// Delete target folder
fs.rmdirSync("target", { recursive: true });
