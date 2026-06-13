// Run with: node examples.js

// --- Basic get/set traps + Reflect default behavior ---
console.log("=== Basic Proxy with get/set traps ===");
{
  const target = { name: "Ada" };
  const proxy = new Proxy(target, {
    get(obj, prop, receiver) {
      console.log(`  get "${String(prop)}"`);
      return Reflect.get(obj, prop, receiver);
    },
    set(obj, prop, value, receiver) {
      console.log(`  set "${String(prop)}" = ${value}`);
      return Reflect.set(obj, prop, value, receiver);
    },
  });
  proxy.name; // logs "get name"
  proxy.age = 30; // logs "set age = 30"
  console.log("target.age:", target.age); // 30 -- writes go through to target
}

// --- has / deleteProperty / ownKeys traps ---
console.log("\n=== has, deleteProperty, ownKeys traps ===");
{
  const secret = { password: "hunter2", username: "ada" };
  const hideSecrets = new Proxy(secret, {
    has(target, prop) {
      return prop !== "password" && Reflect.has(target, prop);
    },
    ownKeys(target) {
      return Reflect.ownKeys(target).filter((k) => k !== "password");
    },
    deleteProperty(target, prop) {
      if (prop === "password") throw new TypeError("Cannot delete password");
      return Reflect.deleteProperty(target, prop);
    },
  });
  console.log("'password' in proxy:", "password" in hideSecrets); // false
  console.log("'username' in proxy:", "username" in hideSecrets); // true
  console.log("Object.keys(proxy):", Object.keys(hideSecrets)); // ['username']
  console.log("proxy.password (still readable):", hideSecrets.password); // "hunter2" -- get not trapped
}

// --- Revocable proxy ---
console.log("\n=== Proxy.revocable ===");
{
  const { proxy, revoke } = Proxy.revocable({ value: 42 }, {});
  console.log("before revoke:", proxy.value); // 42
  revoke();
  try {
    proxy.value;
  } catch (err) {
    console.log("after revoke:", err.constructor.name); // TypeError
  }
}

// --- Symbol.toPrimitive ---
console.log("\n=== Symbol.toPrimitive ===");
{
  const money = {
    amount: 10,
    [Symbol.toPrimitive](hint) {
      if (hint === "string") return `$${this.amount.toFixed(2)}`;
      return this.amount;
    },
  };
  console.log("+money:", +money); // 10
  console.log("`${money}`:", `${money}`); // "$10.00"
  console.log("money + 5:", money + 5); // 15 (default hint -> number)
}

// --- Symbol.toStringTag ---
console.log("\n=== Symbol.toStringTag ===");
{
  class Matrix {
    get [Symbol.toStringTag]() {
      return "Matrix";
    }
  }
  console.log(Object.prototype.toString.call(new Matrix())); // "[object Matrix]"
}

// --- Symbol.hasInstance ---
console.log("\n=== Symbol.hasInstance ===");
{
  const Even = {
    [Symbol.hasInstance](x) {
      return typeof x === "number" && x % 2 === 0;
    },
  };
  console.log("4 instanceof Even:", 4 instanceof Even); // true
  console.log("3 instanceof Even:", 3 instanceof Even); // false
}
