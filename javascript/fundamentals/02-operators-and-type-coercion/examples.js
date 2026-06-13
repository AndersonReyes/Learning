// Run with: node examples.js

// --- Arithmetic ---
console.log("5 % 2:", 5 % 2);
console.log("2 ** 5:", 2 ** 5);

// --- == vs === ---
console.log("1 === 1:", 1 === 1);
console.log("1 === '1':", 1 === "1");
console.log("1 == '1':", 1 == "1");
console.log("0 == false:", 0 == false);
console.log("null == undefined:", null == undefined);
console.log("null === undefined:", null === undefined);
console.log("NaN == NaN:", NaN == NaN);
console.log("NaN === NaN:", NaN === NaN);

// --- Short-circuiting with && / || ---
console.log('"" || "default":', "" || "default");
console.log('"hello" || "default":', "hello" || "default");

let called = false;
const sideEffect = () => {
  called = true;
  return "called!";
};
console.log("0 && sideEffect():", 0 && sideEffect());
console.log("sideEffect was called?", called); // false — short-circuited

// --- Nullish coalescing vs || ---
console.log("0 || 'default':", 0 || "default"); // "default" — 0 is falsy
console.log("0 ?? 'default':", 0 ?? "default"); // 0 — 0 is not nullish
console.log("'' ?? 'default':", "" ?? "default"); // ""
console.log("null ?? 'default':", null ?? "default"); // "default"
console.log("undefined ?? 'default':", undefined ?? "default"); // "default"

// --- Optional chaining ---
const user = { profile: { name: "Ada" } };
console.log("user.profile?.name:", user.profile?.name);
console.log("user.address?.city:", user.address?.city); // undefined, no error

const settings = null;
console.log("settings?.save():", settings?.save()); // undefined, save() never called

// --- Bitwise operators ---
console.log("5 & 3:", 5 & 3);
console.log("5 | 2:", 5 | 2);
console.log("5 ^ 1:", 5 ^ 1);
console.log("~5:", ~5);
console.log("1 << 3:", 1 << 3);
console.log("8 >> 2:", 8 >> 2);

// --- Ternary chaining ---
function gradeLabel(score) {
  return score >= 90 ? "A" : score >= 80 ? "B" : score >= 70 ? "C" : "F";
}
console.log("gradeLabel(95):", gradeLabel(95));
console.log("gradeLabel(82):", gradeLabel(82));
console.log("gradeLabel(50):", gradeLabel(50));
