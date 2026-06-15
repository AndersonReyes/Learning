//! Run with: `cargo run --example examples -p intermediate-08-smart-pointers`

use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

// --- ch.15.1: Box<T> for recursive types --------------------------------------------

/// A classic cons-list. Without `Box`, `Cons(i32, List)` would make `List`
/// infinitely large -- `Box<List>` is a fixed-size pointer, breaking the
/// recursion.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn list_sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + list_sum(rest),
        Nil => 0,
    }
}

// --- ch.15.2: Deref / DerefMut + deref coercion -------------------------------------

/// A minimal smart pointer: `Deref`/`DerefMut` give it `&T`/`&mut T` access,
/// and deref coercion lets method calls on `MyBox<String>` reach `String`'s
/// (and then `str`'s) methods.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

// --- ch.15.3: Drop -- cleanup on scope exit, reverse declaration order --------------

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("  dropping {}", self.0);
    }
}

// --- ch.15.6: Rc<RefCell<T>> + Weak -- a small tree ---------------------------------

struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Weak<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    fn add_child(parent: &Rc<TreeNode>, child: &Rc<TreeNode>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

fn main() {
    // --- Box<T>: recursive cons-list ---
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);
    println!("list_sum(&list) = {}", list_sum(&list));

    // --- MyBox: Deref/DerefMut + deref coercion ---
    let mut my_box = MyBox::new(String::from("hello"));
    println!("\n*my_box = {}", *my_box); // *my_box desugars to *my_box.deref()
    println!("my_box.len() = {}", my_box.len()); // deref coercion: &MyBox<String> -> &String
    my_box.push_str(", world"); // deref_mut coercion for `&mut self` method
    println!("after push_str: {}", *my_box);

    // --- Drop: reverse declaration order ---
    println!("\ncreating guards a, b, c...");
    {
        let _a = Guard("a");
        let _b = Guard("b");
        let _c = Guard("c");
        println!("end of scope, dropping in reverse order:");
    } // prints: dropping c, dropping b, dropping a

    // std::mem::drop forces early cleanup
    println!("\nforcing early drop of `early`:");
    let early = Guard("early");
    std::mem::drop(early);
    println!("(early was already dropped before this line)");

    // --- Rc<T>: shared ownership, strong_count, ptr_eq ---
    println!("\nRc::strong_count / Rc::ptr_eq:");
    let a = Rc::new(String::from("shared"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after b = Rc::clone(&a) = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after c = Rc::clone(&a) = {}", Rc::strong_count(&a));
        println!("Rc::ptr_eq(&a, &c) = {}", Rc::ptr_eq(&a, &c));
    } // c dropped here
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("Rc::ptr_eq(&a, &b) = {}", Rc::ptr_eq(&a, &b));

    // --- RefCell<T> / Cell<T>: interior mutability ---
    println!("\nRefCell<Vec<i32>>:");
    let cell = RefCell::new(vec![1, 2, 3]);
    cell.borrow_mut().push(4);
    println!("cell.borrow() = {:?}", cell.borrow());
    println!("cell.borrow().len() = {}", cell.borrow().len());

    println!("\nCell<i32> (Copy types, no borrow guards):");
    let counter = Cell::new(0);
    counter.set(counter.get() + 1);
    counter.set(counter.get() + 1);
    println!("counter.get() = {}", counter.get());

    // --- Rc<RefCell<T>> + Weak: a small tree with parent links ---
    println!("\nTree with Weak parent pointers:");
    let root = TreeNode::new(0);
    let child = TreeNode::new(1);
    TreeNode::add_child(&root, &child);
    let grandchild = TreeNode::new(2);
    TreeNode::add_child(&child, &grandchild);

    println!("root.value = {}", root.value);
    println!("root.children[0].value = {}", root.children.borrow()[0].value);

    // Walk grandchild -> root via repeated upgrade().
    let mut depth = 0;
    let mut current = Rc::clone(&grandchild);
    loop {
        let parent = current.parent.borrow().upgrade();
        match parent {
            Some(p) => {
                depth += 1;
                current = p;
            }
            None => break,
        }
    }
    println!("grandchild's depth (steps to root) = {}", depth);

    // A Weak whose target was dropped upgrades to None.
    let orphan_parent = TreeNode::new(99);
    let orphan = TreeNode::new(100);
    TreeNode::add_child(&orphan_parent, &orphan);
    let weak_parent = Rc::downgrade(&orphan_parent);
    drop(orphan_parent);
    println!(
        "\nafter dropping orphan_parent, weak_parent.upgrade().is_none() = {}",
        weak_parent.upgrade().is_none()
    );
}
