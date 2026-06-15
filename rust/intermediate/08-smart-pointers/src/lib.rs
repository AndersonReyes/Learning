//! Intermediate 08 — Smart Pointers (`Box`, `Deref`, `Drop`, `Rc`, `RefCell`,
//! `Weak`).
//!
//! `notes.md` covers Book ch. 15: `Box<T>` for recursive types,
//! `Deref`/`DerefMut` + deref coercion, the `Drop` trait and drop order,
//! `Rc<T>` shared ownership, `RefCell<T>` interior mutability, and
//! `Weak<T>` for breaking reference cycles. The 5 exercises below: an
//! `Rc<RefCell<_>>`-backed LRU cache (`get`/`put`), a `Weak`-linked tree
//! (`tree_depth`, `lowest_common_ancestor` via `Rc::ptr_eq`), a
//! `Cell`-counting `Deref`/`DerefMut` smart pointer (`CountedRef`), and an
//! RAII object pool (`Pool`/`PoolGuard`, `acquire` + `Drop`).

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

// --- 1. LruCache -------------------------------------------------------------------

/// A fixed-capacity least-recently-used cache.
///
/// Backed by a `RefCell<VecDeque<(K, V)>>` ordered most-recently-used
/// (front) to least-recently-used (back) — `get`/`put` mutate this ordering
/// through `&self` via `RefCell`'s interior mutability.
pub struct LruCache<K, V> {
    capacity: usize,
    entries: RefCell<VecDeque<(K, V)>>,
}

impl<K: PartialEq, V: Clone> LruCache<K, V> {
    /// Creates an empty cache holding at most `capacity` entries. A
    /// `capacity` of `0` means [`put`](LruCache::put) never stores anything.
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            entries: RefCell::new(VecDeque::new()),
        }
    }

    /// The number of entries currently stored (`<= capacity`).
    pub fn len(&self) -> usize {
        self.entries.borrow().len()
    }

    /// Returns a clone of the value for `key`, if present, and marks it as
    /// the most recently used entry. Returns `None` if `key` isn't present.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_08_smart_pointers::LruCache;
    ///
    /// let cache = LruCache::new(2);
    /// cache.put("a", 1);
    /// cache.put("b", 2);
    /// assert_eq!(cache.get(&"a"), Some(1)); // "a" is now most-recently-used
    /// cache.put("c", 3); // evicts "b" (least-recently-used)
    /// assert_eq!(cache.get(&"b"), None);
    /// assert_eq!(cache.get(&"a"), Some(1));
    /// assert_eq!(cache.get(&"c"), Some(3));
    /// ```
    pub fn get(&self, key: &K) -> Option<V> {
        todo!()
    }

    /// Inserts or updates `key` -> `value`, marking it as the most recently
    /// used entry.
    ///
    /// If `key` already exists, its value is replaced (and it moves to
    /// most-recently-used). If inserting a new key would exceed `capacity`,
    /// the least-recently-used entry is evicted first. If `capacity == 0`,
    /// nothing is stored.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_08_smart_pointers::LruCache;
    ///
    /// let cache = LruCache::new(2);
    /// cache.put("a", 1);
    /// cache.put("b", 2);
    /// cache.put("a", 100); // updates "a" and marks it most-recently-used
    /// cache.put("c", 3); // "b" is now least-recently-used, gets evicted
    /// assert_eq!(cache.get(&"b"), None);
    /// assert_eq!(cache.get(&"a"), Some(100));
    /// assert_eq!(cache.get(&"c"), Some(3));
    /// ```
    pub fn put(&self, key: K, value: V) {
        todo!()
    }
}

// --- 2/3. Tree with Weak parent pointers -------------------------------------------

/// A tree node with owning (`Rc`) links down to children and a non-owning
/// (`Weak`) link up to its parent.
pub struct Node {
    pub value: i32,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Weak<Node>>,
}

impl Node {
    /// Creates a new, parentless, childless node.
    pub fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    /// Attaches `child` to `parent`: `child`'s `parent` weak reference is
    /// set to `parent`, and `child` is appended to `parent`'s `children`.
    pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

/// Returns the depth of `node` in its tree: the root has depth `0`, and
/// each step down via [`Node::add_child`] adds `1`.
///
/// Implement by repeatedly calling `.parent.borrow().upgrade()` until it
/// returns `None` (the root has no parent).
///
/// # Examples
///
/// ```ignore
/// use intermediate_08_smart_pointers::{Node, tree_depth};
///
/// let root = Node::new(0);
/// let child = Node::new(1);
/// Node::add_child(&root, &child);
/// let grandchild = Node::new(2);
/// Node::add_child(&child, &grandchild);
///
/// assert_eq!(tree_depth(&root), 0);
/// assert_eq!(tree_depth(&child), 1);
/// assert_eq!(tree_depth(&grandchild), 2);
/// ```
pub fn tree_depth(node: &Rc<Node>) -> usize {
    todo!()
}

/// Returns the lowest common ancestor of `a` and `b` (the deepest node that
/// is an ancestor of both, where a node counts as its own ancestor), or
/// `None` if `a` and `b` have no common ancestor (different trees).
///
/// Implement by collecting `a`'s ancestor chain (including `a` itself, via
/// repeated `.parent.borrow().upgrade()`), then walking `b`'s ancestor chain
/// (including `b`) and returning the first node that's in `a`'s chain
/// (compare with [`Rc::ptr_eq`], not `==`).
///
/// # Examples
///
/// ```ignore
/// use intermediate_08_smart_pointers::{Node, lowest_common_ancestor};
/// use std::rc::Rc;
///
/// let root = Node::new(0);
/// let left = Node::new(1);
/// let right = Node::new(2);
/// Node::add_child(&root, &left);
/// Node::add_child(&root, &right);
/// let ll = Node::new(3);
/// Node::add_child(&left, &ll);
///
/// assert!(Rc::ptr_eq(&lowest_common_ancestor(&ll, &right).unwrap(), &root));
/// assert!(Rc::ptr_eq(&lowest_common_ancestor(&ll, &left).unwrap(), &left));
/// assert!(Rc::ptr_eq(&lowest_common_ancestor(&ll, &ll).unwrap(), &ll));
///
/// let other = Node::new(99);
/// assert!(lowest_common_ancestor(&ll, &other).is_none());
/// ```
pub fn lowest_common_ancestor(a: &Rc<Node>, b: &Rc<Node>) -> Option<Rc<Node>> {
    todo!()
}

// --- 4. CountedRef -------------------------------------------------------------------

/// A smart pointer to `T` that counts how many times it has been
/// dereferenced (read) and mutably dereferenced (written).
pub struct CountedRef<T> {
    value: T,
    reads: Cell<usize>,
    writes: usize,
}

impl<T> CountedRef<T> {
    /// Wraps `value`, with both counters starting at `0`.
    pub fn new(value: T) -> Self {
        CountedRef {
            value,
            reads: Cell::new(0),
            writes: 0,
        }
    }

    /// Number of times [`Deref::deref`] has been called.
    pub fn read_count(&self) -> usize {
        self.reads.get()
    }

    /// Number of times [`DerefMut::deref_mut`] has been called.
    pub fn write_count(&self) -> usize {
        self.writes
    }
}

/// Implement `deref` to increment `self.reads` (via `Cell::set`, since
/// `deref` only gets `&self`) and return `&self.value`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_08_smart_pointers::CountedRef;
///
/// let counted = CountedRef::new(vec![1, 2, 3]);
/// assert_eq!(counted.read_count(), 0);
/// assert_eq!(counted.len(), 3); // deref coercion calls deref() once
/// assert_eq!(counted.read_count(), 1);
/// assert_eq!(*counted, vec![1, 2, 3]); // explicit deref
/// assert_eq!(counted.read_count(), 2);
/// ```
impl<T> Deref for CountedRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        todo!()
    }
}

/// Implement `deref_mut` to increment `self.writes` (a plain field — `&mut
/// self` already permits this without `Cell`) and return `&mut self.value`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_08_smart_pointers::CountedRef;
///
/// let mut counted = CountedRef::new(vec![1, 2, 3]);
/// counted.push(4); // deref_mut coercion calls deref_mut() once
/// assert_eq!(counted.write_count(), 1);
/// assert_eq!(*counted, vec![1, 2, 3, 4]);
/// ```
impl<T> DerefMut for CountedRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        todo!()
    }
}

// --- 5. Pool / PoolGuard -------------------------------------------------------------

/// A simple object pool: a collection of `T`s that can be checked out via
/// [`acquire`](Pool::acquire) and are automatically returned when the
/// returned [`PoolGuard`] is dropped.
pub struct Pool<T> {
    items: RefCell<Vec<T>>,
}

impl<T> Pool<T> {
    /// Creates a pool pre-populated with `items`.
    pub fn new(items: Vec<T>) -> Self {
        Pool {
            items: RefCell::new(items),
        }
    }

    /// The number of items currently available (not checked out).
    pub fn available(&self) -> usize {
        self.items.borrow().len()
    }

    /// Checks out one item, or `None` if the pool is empty.
    ///
    /// Implement by popping from `self.items` (via `borrow_mut()`) and
    /// wrapping the popped value in a [`PoolGuard`] that borrows `self`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_08_smart_pointers::Pool;
    ///
    /// let pool = Pool::new(vec![1, 2]);
    /// assert_eq!(pool.available(), 2);
    ///
    /// let mut g1 = pool.acquire().unwrap();
    /// let g2 = pool.acquire().unwrap();
    /// assert_eq!(pool.available(), 0);
    /// assert!(pool.acquire().is_none());
    ///
    /// assert_eq!(*g1, 2);
    /// *g1 += 100;
    ///
    /// drop(g1); // returns 102 to the pool
    /// assert_eq!(pool.available(), 1);
    ///
    /// drop(g2); // returns 1 to the pool
    /// assert_eq!(pool.available(), 2);
    /// ```
    pub fn acquire(&self) -> Option<PoolGuard<'_, T>> {
        todo!()
    }
}

/// An RAII guard holding one item checked out of a [`Pool`]. `Deref`s to
/// `T`; returns the item to the pool when dropped.
pub struct PoolGuard<'a, T> {
    value: Option<T>,
    pool: &'a Pool<T>,
}

impl<T> Deref for PoolGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.as_ref().expect("value already returned")
    }
}

impl<T> DerefMut for PoolGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.as_mut().expect("value already returned")
    }
}

/// Implement `drop` to take `self.value` (via `Option::take`) and, if
/// `Some`, push it back onto `self.pool.items` (via `borrow_mut()`).
impl<T> Drop for PoolGuard<'_, T> {
    fn drop(&mut self) {
        todo!()
    }
}
