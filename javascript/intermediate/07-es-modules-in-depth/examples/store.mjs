// Demonstrates "modules are singletons": top-level code runs once, and the
// `count` binding is shared by every importer of this module.
export let count = 0;

export function increment() {
  count++;
}
