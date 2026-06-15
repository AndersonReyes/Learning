//! Advanced 01 — Trait Objects, Dynamic Dispatch & OOP Patterns.
//!
//! `notes.md` covers Book ch. 18: trait objects (`dyn Trait`,
//! `Box<dyn Trait>`), dynamic vs. static dispatch, object safety, the State
//! pattern (`self: Box<Self>` transitions), `std::any::Any` downcasting, and
//! the Decorator pattern via nested trait objects. The 5 exercises below: a
//! recursive expression evaluator over `Box<dyn Expr>`
//! (`eval_all`/`Num`/`Add`/`Sub`/`Mul`/`Div`/`Neg`), a turnstile state
//! machine using `self: Box<Self>` (`run_turnstile`), a heterogeneous
//! `Vec<Box<dyn Task>>` run in stable priority order
//! (`run_tasks_in_priority_order`), `dyn Any`-based downcasting over
//! `Vec<Box<dyn Shape>>` (`count_by_type`), and a notification Decorator
//! chain (`EmailNotifier`/`SmsDecorator`/`LoggingDecorator`).

use std::any::Any;
use std::collections::HashMap;

// --- 1. Expr: recursive expression evaluator ------------------------------------------

/// A node in an arithmetic expression tree, evaluated via dynamic dispatch.
pub trait Expr {
    /// Evaluates this expression (and its subtree), returning `Err` if a
    /// division by zero occurs anywhere underneath.
    fn eval(&self) -> Result<f64, String>;
}

/// A literal value.
pub struct Num(pub f64);

/// `lhs + rhs`.
pub struct Add(pub Box<dyn Expr>, pub Box<dyn Expr>);

/// `lhs - rhs`.
pub struct Sub(pub Box<dyn Expr>, pub Box<dyn Expr>);

/// `lhs * rhs`.
pub struct Mul(pub Box<dyn Expr>, pub Box<dyn Expr>);

/// `lhs / rhs`. Evaluating with `rhs == 0.0` returns
/// `Err("division by zero".to_string())`.
pub struct Div(pub Box<dyn Expr>, pub Box<dyn Expr>);

/// `-inner`.
pub struct Neg(pub Box<dyn Expr>);

impl Expr for Num {
    fn eval(&self) -> Result<f64, String> {
        todo!()
    }
}

impl Expr for Add {
    fn eval(&self) -> Result<f64, String> {
        todo!()
    }
}

impl Expr for Sub {
    fn eval(&self) -> Result<f64, String> {
        todo!()
    }
}

impl Expr for Mul {
    fn eval(&self) -> Result<f64, String> {
        todo!()
    }
}

impl Expr for Div {
    fn eval(&self) -> Result<f64, String> {
        todo!()
    }
}

impl Expr for Neg {
    fn eval(&self) -> Result<f64, String> {
        todo!()
    }
}

/// Evaluates each expression in `exprs`, returning one [`Result`] per
/// expression, in the same order. Errors in one expression don't affect the
/// others.
///
/// # Examples
///
/// ```ignore
/// use advanced_01_trait_objects_and_oop_patterns::{eval_all, Num, Add, Div};
///
/// let exprs: Vec<Box<dyn Expr>> = vec![
///     Box::new(Add(Box::new(Num(2.0)), Box::new(Num(3.0)))), // 5.0
///     Box::new(Div(Box::new(Num(1.0)), Box::new(Num(0.0)))), // division by zero
/// ];
/// assert_eq!(
///     eval_all(exprs),
///     vec![Ok(5.0), Err("division by zero".to_string())]
/// );
/// ```
pub fn eval_all(exprs: Vec<Box<dyn Expr>>) -> Vec<Result<f64, String>> {
    todo!()
}

// --- 2. TurnstileState: State pattern with `self: Box<Self>` ---------------------------

/// An event that can occur at a turnstile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A coin is inserted.
    Coin,
    /// The arm is pushed.
    Push,
}

/// A turnstile state. Transitions consume the current state (`self:
/// Box<Self>`) and produce the next one — the classic "State" OOP pattern.
pub trait TurnstileState {
    /// Consumes the current state, returning the state after `event`.
    fn handle(self: Box<Self>, event: Event) -> Box<dyn TurnstileState>;

    /// This state's name, for inspection.
    fn name(&self) -> &'static str;
}

/// The turnstile is locked: a [`Event::Push`] does nothing (stays
/// `Locked`); a [`Event::Coin`] unlocks it.
pub struct Locked;

/// The turnstile is unlocked: a [`Event::Push`] locks it again; an extra
/// [`Event::Coin`] is accepted but changes nothing (stays `Unlocked`).
pub struct Unlocked;

impl TurnstileState for Locked {
    fn handle(self: Box<Self>, event: Event) -> Box<dyn TurnstileState> {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

impl TurnstileState for Unlocked {
    fn handle(self: Box<Self>, event: Event) -> Box<dyn TurnstileState> {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

/// Starts a turnstile in the [`Locked`] state and feeds it `events` in
/// order, returning the state's `name()` *after* each event.
///
/// # Examples
///
/// ```ignore
/// use advanced_01_trait_objects_and_oop_patterns::{run_turnstile, Event};
///
/// assert_eq!(run_turnstile(&[]), Vec::<&str>::new());
/// assert_eq!(run_turnstile(&[Event::Push]), vec!["Locked"]);
/// assert_eq!(run_turnstile(&[Event::Coin]), vec!["Unlocked"]);
/// assert_eq!(run_turnstile(&[Event::Coin, Event::Push]), vec!["Unlocked", "Locked"]);
/// assert_eq!(
///     run_turnstile(&[Event::Coin, Event::Coin, Event::Push, Event::Push]),
///     vec!["Unlocked", "Unlocked", "Locked", "Locked"]
/// );
/// ```
pub fn run_turnstile(events: &[Event]) -> Vec<&'static str> {
    todo!()
}

// --- 3. Task: heterogeneous Vec<Box<dyn Task>> run in stable priority order -------------

/// A unit of work with a priority. Lower numbers run earlier; ties between
/// equal priorities preserve the original `Vec` order (stable sort).
pub trait Task {
    /// This task's priority — lower runs earlier.
    fn priority(&self) -> i32;

    /// Executes the task, returning a label describing what it did.
    fn run(&self) -> String;
}

/// Sorts `tasks` by [`Task::priority`] ascending (stable — ties keep
/// original relative order), then runs each in that order, collecting the
/// labels returned by [`Task::run`].
///
/// # Examples
///
/// ```ignore
/// use advanced_01_trait_objects_and_oop_patterns::{run_tasks_in_priority_order, Task};
///
/// struct Labeled { label: &'static str, priority: i32 }
/// impl Task for Labeled {
///     fn priority(&self) -> i32 { self.priority }
///     fn run(&self) -> String { self.label.to_string() }
/// }
///
/// let tasks: Vec<Box<dyn Task>> = vec![
///     Box::new(Labeled { label: "low", priority: 5 }),
///     Box::new(Labeled { label: "high", priority: 1 }),
///     Box::new(Labeled { label: "also-high", priority: 1 }),
/// ];
/// assert_eq!(
///     run_tasks_in_priority_order(tasks),
///     vec!["high", "also-high", "low"]
/// );
/// ```
pub fn run_tasks_in_priority_order(tasks: Vec<Box<dyn Task>>) -> Vec<String> {
    todo!()
}

// --- 4. Shape: dyn Any downcasting -------------------------------------------------------

/// A shape that can be inspected at runtime via [`Any`].
pub trait Shape: Any {
    /// This shape's area.
    fn area(&self) -> f64;

    /// Returns `self` as `&dyn Any`, enabling downcasting back to the
    /// concrete type via [`Any::downcast_ref`].
    fn as_any(&self) -> &dyn Any;
}

/// A circle with the given `radius`.
pub struct Circle {
    pub radius: f64,
}

/// A square with the given `side` length.
pub struct Square {
    pub side: f64,
}

/// A rectangle with the given `width` and `height`.
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

/// Counts how many shapes in `shapes` are each concrete type, keyed by
/// `"Circle"`, `"Square"`, or `"Rectangle"`. Types with a count of zero are
/// absent from the map (not present with value `0`).
///
/// # Examples
///
/// ```ignore
/// use advanced_01_trait_objects_and_oop_patterns::{count_by_type, Circle, Square, Shape};
///
/// let shapes: Vec<Box<dyn Shape>> = vec![
///     Box::new(Circle { radius: 1.0 }),
///     Box::new(Circle { radius: 2.0 }),
///     Box::new(Square { side: 3.0 }),
/// ];
/// let counts = count_by_type(&shapes);
/// assert_eq!(counts.get("Circle"), Some(&2));
/// assert_eq!(counts.get("Square"), Some(&1));
/// assert_eq!(counts.get("Rectangle"), None);
/// ```
pub fn count_by_type(shapes: &[Box<dyn Shape>]) -> HashMap<&'static str, usize> {
    todo!()
}

// --- 5. Notifier: Decorator pattern via nested trait objects -----------------------------

/// Sends notifications, possibly wrapping another `Notifier` to add
/// behavior (the Decorator pattern).
pub trait Notifier {
    /// Sends `message`, returning the log of every message actually sent
    /// (in the order they were "sent"): this layer's own message (if any)
    /// followed by whatever the wrapped notifier (if any) returns.
    fn send(&self, message: &str) -> Vec<String>;
}

/// The base notifier: sends exactly one message, `"email: {message}"`.
pub struct EmailNotifier;

/// Wraps another [`Notifier`], additionally sending `"sms: {message}"`
/// *before* delegating to the wrapped notifier.
pub struct SmsDecorator {
    pub inner: Box<dyn Notifier>,
}

/// Wraps another [`Notifier`], additionally sending `"log: {message}"`
/// *before* delegating to the wrapped notifier.
pub struct LoggingDecorator {
    pub inner: Box<dyn Notifier>,
}

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) -> Vec<String> {
        todo!()
    }
}

impl Notifier for SmsDecorator {
    fn send(&self, message: &str) -> Vec<String> {
        todo!()
    }
}

impl Notifier for LoggingDecorator {
    fn send(&self, message: &str) -> Vec<String> {
        todo!()
    }
}
