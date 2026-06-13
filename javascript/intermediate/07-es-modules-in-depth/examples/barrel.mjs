// "Barrel" file: aggregates other modules' exports without importing them
// into local scope.
export * from "./math.mjs"; // re-export ALL named exports (PI, square, E) — NOT the default
export { square as sq } from "./math.mjs"; // re-export one, renamed
export { default as log } from "./logger.mjs"; // re-export a default export under a named export
