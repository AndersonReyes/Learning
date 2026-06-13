/**
 * Create an "animal" object using Object.create — NO `class` keyword.
 *
 * Returns an object with:
 *   - an own property `name` set to the given `name`
 *   - a method `speak()` returning `${name} says ${sound}`
 *
 * `speak` must be defined ONCE on a shared prototype object (not as an own
 * property on each returned object) — every animal created by this factory
 * shares the same `speak` function via its prototype.
 *
 * const dog = createAnimal("Rex", "Woof");
 * dog.name              -> "Rex"
 * dog.speak()           -> "Rex says Woof"
 * dog.hasOwnProperty("speak") -> false (it's on the prototype)
 *
 * @param {string} name
 * @param {string} sound
 * @returns {{ name: string, speak: () => string }}
 */
export function createAnimal(name, sound) {
  throw new Error("Not implemented");
}

/**
 * Abstract base class for 2D shapes.
 *
 * Subclasses must override `area()` and `perimeter()`. `describe()` is
 * already implemented here — it only depends on `area()`/`perimeter()`,
 * whichever (possibly overridden) versions `this` resolves to.
 */
export class Shape {
  /**
   * @returns {number} the shape's area
   */
  area() {
    throw new Error("Not implemented");
  }

  /**
   * @returns {number} the shape's perimeter
   */
  perimeter() {
    throw new Error("Not implemented");
  }

  /**
   * Describe this shape using its area and perimeter, each formatted to 2
   * decimal places.
   *
   * new Circle(2).describe()
   *   -> "Circle: area=12.57, perimeter=12.57"
   *
   * @returns {string}
   */
  describe() {
    return `${this.constructor.name}: area=${this.area().toFixed(2)}, perimeter=${this.perimeter().toFixed(2)}`;
  }
}

/**
 * A circle, defined by its radius.
 */
export class Circle extends Shape {
  /**
   * @param {number} radius
   */
  constructor(radius) {
    super();
    this.radius = radius;
  }

  /**
   * @returns {number} pi * radius^2
   */
  area() {
    throw new Error("Not implemented");
  }

  /**
   * @returns {number} 2 * pi * radius
   */
  perimeter() {
    throw new Error("Not implemented");
  }
}

/**
 * A rectangle, defined by its width and height.
 */
export class Rectangle extends Shape {
  /**
   * @param {number} width
   * @param {number} height
   */
  constructor(width, height) {
    super();
    this.width = width;
    this.height = height;
  }

  /**
   * @returns {number} width * height
   */
  area() {
    throw new Error("Not implemented");
  }

  /**
   * @returns {number} 2 * (width + height)
   */
  perimeter() {
    throw new Error("Not implemented");
  }
}

/**
 * Copy each method from `mixin` onto `TargetClass.prototype`, but only if
 * `TargetClass.prototype` does not already define a property of that name.
 * If `mixin` has a key that already exists on `TargetClass.prototype`, throw
 * new Error(`Cannot overwrite existing property: ${key}`) — and apply none
 * of the mixin's methods in that case (fail before mutating).
 *
 * Returns `TargetClass` (same reference) for chaining.
 *
 * class Foo {}
 * applyMixin(Foo, { greet() { return "hi"; } });
 * new Foo().greet() -> "hi"
 *
 * applyMixin(Foo, { greet() { return "bye"; } }); // throws — "greet" exists
 *
 * @param {Function} TargetClass
 * @param {Object<string, Function>} mixin
 * @returns {Function}
 */
export function applyMixin(TargetClass, mixin) {
  throw new Error("Not implemented");
}
