import { describe, test } from "node:test";
import assert from "node:assert/strict";
import { createAnimal, Shape, Circle, Rectangle, applyMixin } from "./exercise.js";

describe("createAnimal", () => {
  test("returns an object with own `name` and inherited `speak`", () => {
    const dog = createAnimal("Rex", "Woof");
    assert.equal(dog.name, "Rex");
    assert.equal(dog.speak(), "Rex says Woof");
    assert.equal(Object.prototype.hasOwnProperty.call(dog, "name"), true);
    assert.equal(Object.prototype.hasOwnProperty.call(dog, "speak"), false);
  });

  test("speak is defined once on a shared prototype, not per instance", () => {
    const dog = createAnimal("Rex", "Woof");
    const cat = createAnimal("Whiskers", "Meow");

    assert.equal(Object.getPrototypeOf(dog), Object.getPrototypeOf(cat));
    assert.equal(Object.getPrototypeOf(dog).speak, dog.speak);
    assert.equal(cat.speak(), "Whiskers says Meow");
  });

  test("different instances have independent own `name` properties", () => {
    const dog = createAnimal("Rex", "Woof");
    const cat = createAnimal("Whiskers", "Meow");
    assert.notEqual(dog.name, cat.name);
  });
});

describe("Shape (abstract base)", () => {
  test("area/perimeter throw on a plain Shape", () => {
    const shape = new Shape();
    assert.throws(() => shape.area(), /Not implemented/);
    assert.throws(() => shape.perimeter(), /Not implemented/);
  });
});

describe("Circle", () => {
  test("area is pi * r^2", () => {
    const circle = new Circle(2);
    assert.equal(circle.area(), Math.PI * 4);
  });

  test("perimeter is 2 * pi * r", () => {
    const circle = new Circle(2);
    assert.equal(circle.perimeter(), 2 * Math.PI * 2);
  });

  test("describe formats area and perimeter to 2 decimals", () => {
    const circle = new Circle(2);
    assert.equal(circle.describe(), "Circle: area=12.57, perimeter=12.57");
  });

  test("is a Shape via the prototype chain", () => {
    const circle = new Circle(1);
    assert.equal(circle instanceof Circle, true);
    assert.equal(circle instanceof Shape, true);
  });
});

describe("Rectangle", () => {
  test("area is width * height", () => {
    const rect = new Rectangle(3, 4);
    assert.equal(rect.area(), 12);
  });

  test("perimeter is 2 * (width + height)", () => {
    const rect = new Rectangle(3, 4);
    assert.equal(rect.perimeter(), 14);
  });

  test("describe formats area and perimeter to 2 decimals", () => {
    const rect = new Rectangle(3, 4);
    assert.equal(rect.describe(), "Rectangle: area=12.00, perimeter=14.00");
  });

  test("is a Shape via the prototype chain", () => {
    const rect = new Rectangle(1, 1);
    assert.equal(rect instanceof Rectangle, true);
    assert.equal(rect instanceof Shape, true);
  });

  test("a non-square rectangle with decimal dimensions", () => {
    const rect = new Rectangle(2.5, 4);
    assert.equal(rect.area(), 10);
    assert.equal(rect.perimeter(), 13);
    assert.equal(rect.describe(), "Rectangle: area=10.00, perimeter=13.00");
  });
});

describe("applyMixin", () => {
  test("adds a new method to TargetClass.prototype, usable by instances", () => {
    const circle = new Circle(1);
    assert.equal(typeof circle.serialize, "undefined");

    const Result = applyMixin(Circle, {
      serialize() {
        return JSON.stringify(this);
      },
    });

    assert.equal(Result, Circle);
    assert.equal(typeof circle.serialize, "function");
    assert.equal(circle.serialize(), JSON.stringify({ radius: 1 }));
  });

  test("existing methods are untouched", () => {
    const rect = new Rectangle(3, 4);
    assert.equal(rect.area(), 12);

    applyMixin(Rectangle, {
      describeShort() {
        return `${this.width}x${this.height}`;
      },
    });

    assert.equal(rect.area(), 12);
    assert.equal(rect.describeShort(), "3x4");
  });

  test("throws when mixin would overwrite an existing property", () => {
    assert.throws(() => {
      applyMixin(Rectangle, {
        area() {
          return -1;
        },
      });
    }, /area/);

    // existing behavior is preserved — the throwing mixin was not applied
    const rect = new Rectangle(2, 2);
    assert.equal(rect.area(), 4);
  });
});
