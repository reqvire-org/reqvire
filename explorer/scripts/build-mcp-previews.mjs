import path from "node:path";
import {compilePreviews} from "@reqvire-org/ads/scenarios/compile";
await compilePreviews(path.resolve(import.meta.dirname,".."),{watched:process.argv.includes("--watch")});
