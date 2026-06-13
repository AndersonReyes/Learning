import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  createBankAccount,
  once,
  memoize,
  curry2,
  createCounterWithHistory,
} from "./exercise.js";

describe("createBankAccount", () => {
  test("starts with the initial balance", () => {
    const account = createBankAccount(100);
    assert.equal(account.getBalance(), 100);
  });

  test("deposit increases the balance and returns it", () => {
    const account = createBankAccount(100);
    assert.equal(account.deposit(50), 150);
    assert.equal(account.getBalance(), 150);
  });

  test("withdraw decreases the balance and returns it", () => {
    const account = createBankAccount(100);
    assert.equal(account.withdraw(30), 70);
  });

  test("withdraw throws on insufficient funds", () => {
    const account = createBankAccount(100);
    assert.throws(() => account.withdraw(1000), /Insufficient funds/);
  });

  test("deposit/withdraw throw on non-positive amounts", () => {
    const account = createBankAccount(100);
    assert.throws(() => account.deposit(-10), /Invalid amount/);
    assert.throws(() => account.withdraw(0), /Invalid amount/);
  });

  test("balance is not directly accessible", () => {
    const account = createBankAccount(100);
    assert.equal(account.balance, undefined);
  });
});

describe("once", () => {
  test("calls the function only once", () => {
    let calls = 0;
    const init = once(() => {
      calls += 1;
      return calls;
    });

    assert.equal(init(), 1);
    assert.equal(init(), 1);
    assert.equal(init(), 1);
    assert.equal(calls, 1);
  });

  test("caches the result of the first call regardless of later arguments", () => {
    const double = once((x) => x * 2);
    assert.equal(double(5), 10);
    assert.equal(double(100), 10);
  });
});

describe("memoize", () => {
  test("calls the underlying function once per unique argument set", () => {
    let calls = 0;
    const square = memoize((x) => {
      calls += 1;
      return x * x;
    });

    assert.equal(square(5), 25);
    assert.equal(square(5), 25);
    assert.equal(calls, 1);

    assert.equal(square(6), 36);
    assert.equal(calls, 2);
  });
});

describe("curry2", () => {
  test("supports calling with one argument at a time", () => {
    const add = curry2((a, b) => a + b);
    assert.equal(add(2)(3), 5);
  });

  test("supports calling with both arguments at once", () => {
    const add = curry2((a, b) => a + b);
    assert.equal(add(2, 3), 5);
  });

  test("works with other operations", () => {
    const multiply = curry2((a, b) => a * b);
    assert.equal(multiply(4)(5), 20);
    assert.equal(multiply(4, 5), 20);
  });
});

describe("createCounterWithHistory", () => {
  test("starts with the initial value in history", () => {
    const counter = createCounterWithHistory();
    assert.equal(counter.getValue(), 0);
    assert.deepEqual(counter.getHistory(), [0]);
  });

  test("increment/decrement update value and history", () => {
    const counter = createCounterWithHistory(0);
    assert.equal(counter.increment(), 1);
    assert.equal(counter.increment(), 2);
    assert.equal(counter.decrement(), 1);
    assert.deepEqual(counter.getHistory(), [0, 1, 2, 1]);
  });

  test("reset clears history back to the start value", () => {
    const counter = createCounterWithHistory(5);
    counter.increment();
    counter.increment();
    assert.equal(counter.reset(), 5);
    assert.equal(counter.getValue(), 5);
    assert.deepEqual(counter.getHistory(), [5]);
  });
});
