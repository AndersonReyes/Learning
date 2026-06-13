import { describe, test } from "node:test";
import assert from "node:assert/strict";
import { myCall, myApply, myBind, bindAll, createChainable } from "./exercise.js";

describe("myCall", () => {
  test("calls fn with `this` set to an object thisArg", () => {
    function getName() {
      return this.name;
    }
    assert.equal(myCall(getName, { name: "Ada" }), "Ada");
  });

  test("passes individual arguments through", () => {
    function add(a, b) {
      return a + b;
    }
    assert.equal(myCall(add, null, 1, 2), 3);
  });

  test("returns fn's return value", () => {
    function makeGreeting(greeting) {
      return `${greeting}, ${this.name}`;
    }
    assert.equal(myCall(makeGreeting, { name: "Ada" }, "Hi"), "Hi, Ada");
  });

  test("null/undefined thisArg -> `this` is undefined inside fn", () => {
    function getThis() {
      return this;
    }
    assert.equal(myCall(getThis, null), undefined);
    assert.equal(myCall(getThis, undefined), undefined);
  });

  test("primitive thisArg is boxed -- `this` is a wrapper object", () => {
    function getThisInfo() {
      return { type: typeof this, value: this.valueOf() };
    }
    assert.deepEqual(myCall(getThisInfo, 5), { type: "object", value: 5 });
    assert.deepEqual(myCall(getThisInfo, "hi"), { type: "object", value: "hi" });
  });

  test("object thisArg is passed through by reference (not boxed)", () => {
    function setName(name) {
      this.name = name;
      return this;
    }
    const target = {};
    const result = myCall(setName, target, "Grace");
    assert.equal(result, target);
    assert.equal(target.name, "Grace");
  });

  test("does not leave the function attached to thisArg afterwards", () => {
    const obj = { name: "Ada" };
    function getName() {
      return this.name;
    }
    myCall(getName, obj);
    assert.deepEqual(Object.keys(obj), ["name"]);
  });
});

describe("myApply", () => {
  test("passes arguments from an array", () => {
    function add(a, b) {
      return a + b;
    }
    assert.equal(myApply(add, null, [1, 2]), 3);
  });

  test("defaults to no arguments when argsArray is omitted", () => {
    function getArgCount() {
      return arguments.length;
    }
    assert.equal(myApply(getArgCount, null), 0);
    assert.equal(myApply(getArgCount, null, undefined), 0);
  });

  test("sets `this` to an object thisArg", () => {
    function getName() {
      return this.name;
    }
    assert.equal(myApply(getName, { name: "Ada" }, []), "Ada");
  });

  test("primitive thisArg is boxed", () => {
    function getType() {
      return typeof this;
    }
    assert.equal(myApply(getType, 42, []), "object");
  });
});

describe("myBind", () => {
  test("binds `this` for a later call", () => {
    function getName() {
      return this.name;
    }
    const bound = myBind(getName, { name: "Ada" });
    assert.equal(bound(), "Ada");
  });

  test("partially applies leading arguments", () => {
    const add = (a, b, c) => a + b + c;
    const add5 = myBind(add, null, 5);
    assert.equal(add5(2, 3), 10);
  });

  test("combines bound args with call-time args in order", () => {
    function combine(...args) {
      return args.join("-");
    }
    const bound = myBind(combine, null, "a", "b");
    assert.equal(bound("c", "d"), "a-b-c-d");
  });

  test("returns fn's return value", () => {
    function makeGreeting(greeting, punctuation) {
      return `${greeting}, ${this.name}${punctuation}`;
    }
    const greetAda = myBind(makeGreeting, { name: "Ada" }, "Hi");
    assert.equal(greetAda("!"), "Hi, Ada!");
  });

  test("the returned function can be called multiple times", () => {
    function increment() {
      this.count += 1;
      return this.count;
    }
    const obj = { count: 0 };
    const bound = myBind(increment, obj);
    assert.equal(bound(), 1);
    assert.equal(bound(), 2);
    assert.equal(bound(), 3);
  });

  test("fixes the classic 'losing this' callback gotcha", () => {
    const counter = {
      count: 10,
      increment() {
        this.count += 1;
        return this.count;
      },
    };
    const detached = myBind(counter.increment, counter);
    // simulate passing it around as a bare callback
    const callbacks = [detached];
    assert.equal(callbacks[0](), 11);
  });
});

describe("bindAll", () => {
  test("binds named methods so they survive extraction", () => {
    const obj = {
      count: 0,
      increment() {
        this.count += 1;
        return this.count;
      },
    };
    bindAll(obj, ["increment"]);
    const extracted = obj.increment;
    assert.equal(extracted(), 1);
    assert.equal(extracted(), 2);
  });

  test("returns the same object reference", () => {
    const obj = { greet() { return this.name; }, name: "Ada" };
    const result = bindAll(obj, ["greet"]);
    assert.equal(result, obj);
  });

  test("binds multiple methods", () => {
    const obj = {
      x: 1,
      getX() { return this.x; },
      getXDoubled() { return this.x * 2; },
    };
    bindAll(obj, ["getX", "getXDoubled"]);
    const { getX, getXDoubled } = obj;
    assert.equal(getX(), 1);
    assert.equal(getXDoubled(), 2);
  });

  test("throws if a name is not a function property on obj", () => {
    const obj = { count: 0, increment() { this.count += 1; } };
    assert.throws(() => bindAll(obj, ["increment", "missing"]), /missing/);
  });

  test("throws if the named property exists but isn't a function", () => {
    const obj = { count: 0 };
    assert.throws(() => bindAll(obj, ["count"]), /count/);
  });
});

describe("createChainable", () => {
  test("starts at the initial value", () => {
    assert.equal(createChainable(10).value(), 10);
  });

  test("add returns the builder for chaining and updates the value", () => {
    const builder = createChainable(10);
    const result = builder.add(5);
    assert.equal(result, builder);
    assert.equal(builder.value(), 15);
  });

  test("chains multiple operations in order", () => {
    assert.equal(createChainable(10).add(5).multiply(2).value(), 30);
    assert.equal(createChainable(10).subtract(4).divide(2).value(), 3);
  });

  test("subtract and multiply work as expected", () => {
    assert.equal(createChainable(10).subtract(3).value(), 7);
    assert.equal(createChainable(4).multiply(3).value(), 12);
  });

  test("divide performs normal division", () => {
    assert.equal(createChainable(20).divide(4).value(), 5);
  });

  test("divide by zero throws", () => {
    assert.throws(() => createChainable(10).divide(0), /Division by zero/);
  });

  test("independent builders don't share state", () => {
    const a = createChainable(1);
    const b = createChainable(1);
    a.add(10);
    assert.equal(a.value(), 11);
    assert.equal(b.value(), 1);
  });
});
