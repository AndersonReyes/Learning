//! Run with: `cargo run --example examples -p advanced-01-trait-objects-and-oop-patterns`

use std::any::Any;

fn main() {
    // --- Heterogeneous Vec<Box<dyn Trait>> + dynamic dispatch (ch. 18.2) ---------------
    println!("-- trait objects: Vec<Box<dyn Draw>> --");

    trait Draw {
        fn draw(&self) -> String;
    }

    struct Button {
        label: String,
    }
    struct SelectBox {
        options: Vec<String>,
    }

    impl Draw for Button {
        fn draw(&self) -> String {
            format!("[Button: {}]", self.label)
        }
    }
    impl Draw for SelectBox {
        fn draw(&self) -> String {
            format!("[SelectBox: {} options]", self.options.len())
        }
    }

    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            label: "OK".to_string(),
        }),
        Box::new(SelectBox {
            options: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }),
    ];
    for c in &components {
        println!("{}", c.draw());
    }

    // Static dispatch (generic, monomorphized) vs. dynamic dispatch (dyn Trait):
    fn draw_static(c: &impl Draw) -> String {
        c.draw()
    }
    fn draw_dynamic(c: &dyn Draw) -> String {
        c.draw()
    }
    let button = Button {
        label: "Cancel".to_string(),
    };
    println!("static:  {}", draw_static(&button));
    println!("dynamic: {}", draw_dynamic(components[1].as_ref()));

    // --- Default trait methods as "inheritance" (ch. 18.1) -----------------------------
    println!("\n-- default trait methods --");

    trait Greet {
        fn name(&self) -> &str;
        // Default method built on top of the required `name` method.
        fn greet(&self) -> String {
            format!("Hello, {}!", self.name())
        }
    }

    struct Person {
        name: String,
    }
    impl Greet for Person {
        fn name(&self) -> &str {
            &self.name
        }
    }
    struct Robot;
    impl Greet for Robot {
        fn name(&self) -> &str {
            "Robot"
        }
        // Override the default.
        fn greet(&self) -> String {
            "BEEP BOOP".to_string()
        }
    }

    let person = Person {
        name: "Ada".to_string(),
    };
    println!("{}", person.greet());
    println!("{}", Robot.greet());

    // --- State pattern with `self: Box<Self>` (ch. 18.3) -------------------------------
    println!("\n-- State pattern --");

    trait LightState {
        fn next(self: Box<Self>) -> Box<dyn LightState>;
        fn name(&self) -> &'static str;
    }

    struct Red;
    struct Green;
    struct Yellow;

    impl LightState for Red {
        fn next(self: Box<Self>) -> Box<dyn LightState> {
            Box::new(Green)
        }
        fn name(&self) -> &'static str {
            "Red"
        }
    }
    impl LightState for Green {
        fn next(self: Box<Self>) -> Box<dyn LightState> {
            Box::new(Yellow)
        }
        fn name(&self) -> &'static str {
            "Green"
        }
    }
    impl LightState for Yellow {
        fn next(self: Box<Self>) -> Box<dyn LightState> {
            Box::new(Red)
        }
        fn name(&self) -> &'static str {
            "Yellow"
        }
    }

    let mut light: Box<dyn LightState> = Box::new(Red);
    for _ in 0..5 {
        print!("{} -> ", light.name());
        light = light.next();
    }
    println!("{}", light.name());

    // --- dyn Any downcasting --------------------------------------------------------------
    println!("\n-- dyn Any downcasting --");

    trait Animal: Any {
        fn sound(&self) -> &str;
        fn as_any(&self) -> &dyn Any;
    }

    struct Dog;
    struct Cat;
    impl Animal for Dog {
        fn sound(&self) -> &str {
            "Woof"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl Animal for Cat {
        fn sound(&self) -> &str {
            "Meow"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in &animals {
        let kind = if animal.as_any().downcast_ref::<Dog>().is_some() {
            "Dog"
        } else if animal.as_any().downcast_ref::<Cat>().is_some() {
            "Cat"
        } else {
            "Unknown"
        };
        println!("{kind} says {}", animal.sound());
    }

    // --- Decorator pattern via nested trait objects -------------------------------------
    println!("\n-- Decorator pattern --");

    trait Notifier {
        fn send(&self, msg: &str) -> Vec<String>;
    }

    struct Email;
    impl Notifier for Email {
        fn send(&self, msg: &str) -> Vec<String> {
            vec![format!("email: {msg}")]
        }
    }

    struct WithSms {
        inner: Box<dyn Notifier>,
    }
    impl Notifier for WithSms {
        fn send(&self, msg: &str) -> Vec<String> {
            let mut out = vec![format!("sms: {msg}")];
            out.extend(self.inner.send(msg));
            out
        }
    }

    let notifier: Box<dyn Notifier> = Box::new(WithSms {
        inner: Box::new(Email),
    });
    for line in notifier.send("server down") {
        println!("{line}");
    }
}
