import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  deepFreeze,
  getAllPropertyNames,
  pickEnumerable,
  lockShape,
  groupKeysByOrigin,
} from "./exercise.js";

describe("deepFreeze", () => {
  test("returns the same reference, mutated to frozen", () => {
    const obj = { a: 1 };
    assert.equal(deepFreeze(obj), obj);
    assert.ok(Object.isFrozen(obj));
  });

  test("freezes nested objects and arrays", () => {
    const obj = { a: 1, b: { c: 2 }, arr: [1, { x: 1 }] };
    deepFreeze(obj);
    assert.ok(Object.isFrozen(obj.b));
    assert.ok(Object.isFrozen(obj.arr));
    assert.ok(Object.isFrozen(obj.arr[1]));
  });

  test("nested mutation throws TypeError after deep freeze", () => {
    const obj = { b: { c: 2 } };
    deepFreeze(obj);
    assert.throws(() => {
      obj.b.c = 99;
    }, TypeError);
  });

  test("primitives and null pass through unchanged", () => {
    assert.equal(deepFreeze(5), 5);
    assert.equal(deepFreeze(null), null);
    assert.equal(deepFreeze("str"), "str");
    assert.equal(deepFreeze(undefined), undefined);
  });

  test("handles circular references without infinite looping", () => {
    const a = {};
    a.self = a;
    deepFreeze(a);
    assert.ok(Object.isFrozen(a));
  });

  test("handles mutual circular references", () => {
    const a = {};
    const b = { a };
    a.b = b;
    deepFreeze(a);
    assert.ok(Object.isFrozen(a));
    assert.ok(Object.isFrozen(b));
  });
});

describe("getAllPropertyNames", () => {
  class Animal {
    constructor(name) {
      this.name = name;
    }
    speak() {
      return "...";
    }
  }
  class Dog extends Animal {
    constructor(name) {
      super(name);
      this.breed = "?";
    }
    bark() {
      return "Woof";
    }
  }

  test("walks the prototype chain, own names first, deduped, stops before Object.prototype", () => {
    const result = getAllPropertyNames(new Dog("Rex"));
    assert.deepEqual(result, ["name", "breed", "constructor", "bark", "speak"]);
  });

  test("plain object returns just its own keys", () => {
    assert.deepEqual(getAllPropertyNames({ a: 1, b: 2 }), ["a", "b"]);
  });

  test("includes non-enumerable own properties", () => {
    const obj = {};
    Object.defineProperty(obj, "hidden", { value: 1, enumerable: false });
    assert.deepEqual(getAllPropertyNames(obj), ["hidden"]);
  });

  test("handles Object.create(null) without throwing", () => {
    const obj = Object.create(null);
    obj.x = 1;
    assert.deepEqual(getAllPropertyNames(obj), ["x"]);
  });

  test("empty object returns an empty array", () => {
    assert.deepEqual(getAllPropertyNames({}), []);
  });
});

describe("pickEnumerable", () => {
  test("picks only the requested own-enumerable keys", () => {
    assert.deepEqual(pickEnumerable({ a: 1, b: 2, c: 3 }, ["a", "c"]), {
      a: 1,
      c: 3,
    });
  });

  test("skips keys that don't exist on the object", () => {
    assert.deepEqual(pickEnumerable({ a: 1 }, ["a", "missing"]), { a: 1 });
  });

  test("skips own non-enumerable properties", () => {
    const obj = { a: 1 };
    Object.defineProperty(obj, "hidden", { value: 2, enumerable: false });
    assert.deepEqual(pickEnumerable(obj, ["a", "hidden"]), { a: 1 });
  });

  test("invokes getters and copies their current value", () => {
    const obj = {
      get computed() {
        return 42;
      },
    };
    assert.deepEqual(pickEnumerable(obj, ["computed"]), { computed: 42 });
  });

  test("skips inherited enumerable properties (not own)", () => {
    const proto = { inherited: 1 };
    const child = Object.create(proto);
    child.own = 2;
    assert.deepEqual(pickEnumerable(child, ["own", "inherited"]), { own: 2 });
  });

  test("empty keys array returns an empty object", () => {
    assert.deepEqual(pickEnumerable({ a: 1 }, []), {});
  });
});

describe("lockShape", () => {
  test("returns the same object", () => {
    const obj = { a: 1 };
    assert.equal(lockShape(obj), obj);
  });

  test("existing writable properties can still be reassigned", () => {
    const obj = { a: 1, b: 2 };
    lockShape(obj);
    obj.a = 99;
    assert.equal(obj.a, 99);
  });

  test("adding a new property throws TypeError (non-extensible)", () => {
    const obj = { a: 1 };
    lockShape(obj);
    assert.throws(() => {
      obj.c = 3;
    }, TypeError);
    assert.equal("c" in obj, false);
  });

  test("deleting an existing property throws TypeError (non-configurable)", () => {
    const obj = { a: 1 };
    lockShape(obj);
    assert.throws(() => {
      delete obj.a;
    }, TypeError);
  });

  test("redefining enumerable/configurable on a locked property throws TypeError", () => {
    const obj = { a: 1 };
    lockShape(obj);
    assert.throws(() => Object.defineProperty(obj, "a", { enumerable: false }), TypeError);
  });

  test("object becomes non-extensible", () => {
    const obj = { a: 1 };
    lockShape(obj);
    assert.equal(Object.isExtensible(obj), false);
  });

  test("preserves getters/setters, which also become non-configurable", () => {
    const obj = {
      get x() {
        return 1;
      },
    };
    lockShape(obj);
    assert.equal(obj.x, 1);
    assert.throws(() => {
      delete obj.x;
    }, TypeError);
  });
});

describe("groupKeysByOrigin", () => {
  test("categorizes own-enumerable, own-non-enumerable, and inherited-enumerable keys", () => {
    const proto = {};
    Object.defineProperty(proto, "protoHidden", { value: "h", enumerable: false });
    proto.protoVisible = "v";
    const obj = Object.create(proto);
    obj.ownVisible = "a";
    Object.defineProperty(obj, "ownHidden", { value: "b", enumerable: false });

    assert.deepEqual(groupKeysByOrigin(obj), {
      ownEnumerable: ["ownVisible"],
      ownNonEnumerable: ["ownHidden"],
      inheritedEnumerable: ["protoVisible"],
    });
  });

  test("plain object with only own-enumerable keys", () => {
    assert.deepEqual(groupKeysByOrigin({ a: 1, b: 2 }), {
      ownEnumerable: ["a", "b"],
      ownNonEnumerable: [],
      inheritedEnumerable: [],
    });
  });

  test("empty object returns all-empty arrays", () => {
    assert.deepEqual(groupKeysByOrigin({}), {
      ownEnumerable: [],
      ownNonEnumerable: [],
      inheritedEnumerable: [],
    });
  });

  test("Object.create(null) with no inherited properties", () => {
    const obj = Object.create(null);
    obj.a = 1;
    assert.deepEqual(groupKeysByOrigin(obj), {
      ownEnumerable: ["a"],
      ownNonEnumerable: [],
      inheritedEnumerable: [],
    });
  });
});
