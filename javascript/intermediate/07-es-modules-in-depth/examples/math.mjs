// Named exports + a default export in the same module.
export const PI = 3.14159;

export function square(x) {
  return x * x;
}

const E = 2.71828;
export { E };

export default function describe(x) {
  return `square(${x}) = ${square(x)}`;
}
