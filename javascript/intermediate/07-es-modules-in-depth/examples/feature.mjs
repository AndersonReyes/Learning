// A module loaded only via dynamic import() in examples.js, demonstrating
// conditional/lazy loading.
export function run() {
  return "feature ran";
}

export default function defaultRun() {
  return "default feature ran";
}
