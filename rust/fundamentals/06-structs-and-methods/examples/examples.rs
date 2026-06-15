//! Run with: `cargo run --example examples -p fundamentals-06-structs-and-methods`
//!
//! Demonstrates `notes.md`'s struct/method concepts: defining and
//! instantiating structs, field init shorthand, struct update syntax,
//! tuple structs, unit-like structs, `&self`/`&mut self`/`self` methods,
//! associated functions, automatic referencing, and `#[derive(Debug)]`.
//! Self-contained: doesn't call into this package's exercises (those are
//! unimplemented `todo!()` stubs until you finish them).

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Field init shorthand: `email`/`username` match the field names.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Associated function (constructor) — called as `Rectangle::new(..)`.
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    /// Another associated function, returning a square `Rectangle`.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    /// `&self` — borrows the instance immutably.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// `&self` — can call other `&self` methods on `self`.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// `&mut self` — required to modify fields.
    fn double_width(&mut self) {
        self.width *= 2;
    }

    /// `self` (by value) — consumes the instance, returns a new one.
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }
}

// Tuple structs: distinct types even with identical underlying layout.
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

// Unit-like struct: no fields, useful as a marker type.
struct AlwaysEqual;

fn main() {
    // --- Instantiation, field init shorthand, Debug printing ---

    let user1 = build_user(String::from("alice@example.com"), String::from("alice"));
    println!("user1 = {user1:?}");

    // Direct field access via dot notation.
    println!(
        "user1.username = {}, user1.email = {}, user1.active = {}, user1.sign_in_count = {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    // --- Struct update syntax ---
    // `..user1` fills in the remaining fields, MOVING non-Copy fields
    // (here, `username` and `email`) out of `user1`.
    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1
    };
    println!("user2 = {user2:?}");
    // user1.username and user1.email are no longer accessible (moved),
    // but user1.active and user1.sign_in_count (both Copy) still would be
    // if we hadn't moved the whole binding's fields above.

    // --- Associated functions as constructors ---

    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let sq = Rectangle::square(20);
    println!("rect1 = {rect1:?}, area = {}", rect1.area());
    println!("sq = {sq:?}, area = {}", sq.area());

    // --- Automatic referencing: `rect1.area()` is `(&rect1).area()` ---

    println!("rect1.can_hold(&rect2) = {}", rect1.can_hold(&rect2));
    println!("rect2.can_hold(&rect1) = {}", rect2.can_hold(&rect1));

    // --- &mut self ---

    let mut rect3 = Rectangle::new(5, 10);
    rect3.double_width();
    println!("rect3 after double_width = {rect3:?}");

    // --- self by value: consumes rect3, returns a new Rectangle ---

    let squared = rect3.into_square();
    println!("squared = {squared:?}");
    // rect3 is no longer usable here — it was moved into `into_square`.

    // --- Tuple structs: distinct types, accessed via .0/.1/.2 ---

    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);
    println!("origin = ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    // origin and black are different types even though both wrap (i32, i32, i32).

    // --- Unit-like struct ---

    let _subject = AlwaysEqual;
    println!("AlwaysEqual instance created (no data)");

    // --- dbg! prints to stderr with file:line, and returns ownership ---

    let area = dbg!(rect1.width * rect1.height);
    println!("area computed via dbg! = {area}");
}
