use intermediate_08_smart_pointers::{
    lowest_common_ancestor, tree_depth, CountedRef, LruCache, Node, Pool,
};
use std::rc::Rc;

// --- LruCache -------------------------------------------------------------

#[test]
fn lru_basic_eviction() {
    let cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    assert_eq!(cache.get(&"a"), Some(1)); // "a" is now most-recently-used
    cache.put("c", 3); // evicts "b" (least-recently-used)
    assert_eq!(cache.get(&"b"), None);
    assert_eq!(cache.get(&"a"), Some(1));
    assert_eq!(cache.get(&"c"), Some(3));
}

#[test]
fn lru_update_existing_key_refreshes_recency() {
    let cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("a", 100); // updates "a" and marks it most-recently-used
    cache.put("c", 3); // "b" is now least-recently-used, gets evicted
    assert_eq!(cache.get(&"b"), None);
    assert_eq!(cache.get(&"a"), Some(100));
    assert_eq!(cache.get(&"c"), Some(3));
}

#[test]
fn lru_capacity_zero_stores_nothing() {
    let cache: LruCache<&str, i32> = LruCache::new(0);
    cache.put("a", 1);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.get(&"a"), None);
}

#[test]
fn lru_capacity_one_evicts_previous() {
    let cache = LruCache::new(1);
    cache.put("a", 1);
    assert_eq!(cache.get(&"a"), Some(1));
    cache.put("b", 2); // evicts "a"
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"b"), Some(2));
    assert_eq!(cache.len(), 1);
}

#[test]
fn lru_get_missing_key_returns_none() {
    let cache: LruCache<&str, i32> = LruCache::new(2);
    cache.put("a", 1);
    assert_eq!(cache.get(&"z"), None);
}

#[test]
fn lru_len_tracks_size_up_to_capacity() {
    let cache = LruCache::new(3);
    assert_eq!(cache.len(), 0);
    cache.put("a", 1);
    assert_eq!(cache.len(), 1);
    cache.put("b", 2);
    cache.put("c", 3);
    assert_eq!(cache.len(), 3);
    cache.put("d", 4); // evicts "a", len stays at capacity
    assert_eq!(cache.len(), 3);
}

#[test]
fn lru_update_without_eviction_when_under_capacity() {
    let cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("a", 99); // update, no eviction needed (len 2 <= capacity 3)
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get(&"a"), Some(99));
    assert_eq!(cache.get(&"b"), Some(2));
}

#[test]
fn lru_get_on_already_most_recent_does_not_disturb_order() {
    let cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2); // order (MRU -> LRU): b, a
    cache.get(&"b"); // "b" is already MRU, order unchanged: b, a
    cache.put("c", 3); // evicts "a" (LRU)
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"b"), Some(2));
    assert_eq!(cache.get(&"c"), Some(3));
}

// --- Node: tree_depth / lowest_common_ancestor -----------------------------

/// Builds:
/// ```text
/// root(0)
/// ├── left(1)
/// │   └── ll(3)
/// │       └── lll(4)
/// └── right(2)
/// ```
fn build_tree() -> (Rc<Node>, Rc<Node>, Rc<Node>, Rc<Node>, Rc<Node>) {
    let root = Node::new(0);
    let left = Node::new(1);
    let right = Node::new(2);
    Node::add_child(&root, &left);
    Node::add_child(&root, &right);
    let ll = Node::new(3);
    Node::add_child(&left, &ll);
    let lll = Node::new(4);
    Node::add_child(&ll, &lll);
    (root, left, right, ll, lll)
}

#[test]
fn tree_depth_root_is_zero() {
    let (root, ..) = build_tree();
    assert_eq!(tree_depth(&root), 0);
}

#[test]
fn tree_depth_children_are_one() {
    let (_root, left, right, ..) = build_tree();
    assert_eq!(tree_depth(&left), 1);
    assert_eq!(tree_depth(&right), 1);
}

#[test]
fn tree_depth_grandchild_is_two() {
    let (_root, _, _, ll, _) = build_tree();
    assert_eq!(tree_depth(&ll), 2);
}

#[test]
fn tree_depth_great_grandchild_is_three() {
    let (_root, _, _, _, lll) = build_tree();
    assert_eq!(tree_depth(&lll), 3);
}

#[test]
fn tree_depth_single_node_is_zero() {
    let solo = Node::new(42);
    assert_eq!(tree_depth(&solo), 0);
}

#[test]
fn lca_of_grandchild_and_uncle_is_root() {
    let (root, _, right, ll, _) = build_tree();
    assert!(Rc::ptr_eq(&lowest_common_ancestor(&ll, &right).unwrap(), &root));
}

#[test]
fn lca_of_node_and_its_ancestor_is_the_ancestor() {
    let (_root, left, _, ll, _) = build_tree();
    assert!(Rc::ptr_eq(&lowest_common_ancestor(&ll, &left).unwrap(), &left));
    // order shouldn't matter
    assert!(Rc::ptr_eq(&lowest_common_ancestor(&left, &ll).unwrap(), &left));
}

#[test]
fn lca_of_node_with_itself_is_itself() {
    let (_root, _, _, ll, _) = build_tree();
    assert!(Rc::ptr_eq(&lowest_common_ancestor(&ll, &ll).unwrap(), &ll));
}

#[test]
fn lca_returns_none_for_different_trees() {
    let (_root, _, _, ll, _) = build_tree();
    let other_root = Node::new(99);
    assert!(lowest_common_ancestor(&ll, &other_root).is_none());
}

#[test]
fn lca_of_two_siblings_is_their_parent() {
    let (_root, left, _, ..) = build_tree();
    let a_child = Node::new(10);
    let b_child = Node::new(11);
    Node::add_child(&left, &a_child);
    Node::add_child(&left, &b_child);
    assert!(Rc::ptr_eq(
        &lowest_common_ancestor(&a_child, &b_child).unwrap(),
        &left
    ));
}

#[test]
fn lca_of_deep_node_and_other_branch_is_root() {
    let (root, _, right, _, lll) = build_tree();
    assert!(Rc::ptr_eq(&lowest_common_ancestor(&lll, &right).unwrap(), &root));
}

// --- CountedRef -------------------------------------------------------------

#[test]
fn counted_ref_explicit_deref_increments_read_count() {
    let counted = CountedRef::new(5);
    assert_eq!(counted.read_count(), 0);
    assert_eq!(*counted, 5);
    assert_eq!(counted.read_count(), 1);
}

#[test]
fn counted_ref_deref_coercion_method_call_increments_read_count() {
    let counted = CountedRef::new(vec![1, 2, 3]);
    assert_eq!(counted.len(), 3); // deref coercion calls deref() once
    assert_eq!(counted.read_count(), 1);
}

#[test]
fn counted_ref_multiple_reads_accumulate() {
    let counted = CountedRef::new(vec![1, 2, 3]);
    assert_eq!(counted.len(), 3);
    assert!(!counted.is_empty());
    assert_eq!(*counted, vec![1, 2, 3]);
    assert_eq!(counted.read_count(), 3);
}

#[test]
fn counted_ref_deref_mut_via_method_increments_write_count() {
    let mut counted = CountedRef::new(vec![1, 2, 3]);
    assert_eq!(counted.write_count(), 0);
    counted.push(4); // deref_mut coercion calls deref_mut() once
    assert_eq!(counted.write_count(), 1);
    assert_eq!(*counted, vec![1, 2, 3, 4]);
}

#[test]
fn counted_ref_compound_assign_only_counts_as_write() {
    let mut counted = CountedRef::new(10);
    *counted += 5; // *counted as an assignment target uses deref_mut, not deref
    assert_eq!(counted.write_count(), 1);
    assert_eq!(counted.read_count(), 0);
    assert_eq!(*counted, 15); // this read uses deref
    assert_eq!(counted.read_count(), 1);
}

#[test]
fn counted_ref_works_with_string() {
    let mut counted = CountedRef::new(String::from("hello"));
    assert_eq!(counted.len(), 5); // deref read
    counted.push_str(" world"); // deref_mut write
    assert_eq!(*counted, "hello world".to_string()); // deref read
    assert_eq!(counted.read_count(), 2);
    assert_eq!(counted.write_count(), 1);
}

// --- Pool / PoolGuard --------------------------------------------------------

#[test]
fn pool_acquire_reduces_available() {
    let pool = Pool::new(vec![1, 2, 3]);
    assert_eq!(pool.available(), 3);
    let _g = pool.acquire().unwrap();
    assert_eq!(pool.available(), 2);
}

#[test]
fn pool_acquire_returns_none_when_empty() {
    let pool: Pool<i32> = Pool::new(vec![]);
    assert!(pool.acquire().is_none());
    assert_eq!(pool.available(), 0);
}

#[test]
fn pool_acquire_pops_in_lifo_order() {
    let pool = Pool::new(vec![1, 2, 3]);
    let g1 = pool.acquire().unwrap();
    assert_eq!(*g1, 3);
    let g2 = pool.acquire().unwrap();
    assert_eq!(*g2, 2);
}

#[test]
fn pool_full_lifecycle_mutation_and_return() {
    let pool = Pool::new(vec![1, 2]);
    assert_eq!(pool.available(), 2);

    let mut g1 = pool.acquire().unwrap();
    let g2 = pool.acquire().unwrap();
    assert_eq!(pool.available(), 0);
    assert!(pool.acquire().is_none());

    assert_eq!(*g1, 2);
    *g1 += 100;

    drop(g1); // returns 102 to the pool
    assert_eq!(pool.available(), 1);

    drop(g2); // returns 1 to the pool
    assert_eq!(pool.available(), 2);
}

#[test]
fn pool_mutated_value_persists_across_reacquire() {
    let pool = Pool::new(vec![10]);
    {
        let mut g = pool.acquire().unwrap();
        *g += 5;
        assert_eq!(*g, 15);
    } // dropped here, returns 15 to the pool
    assert_eq!(pool.available(), 1);

    let g2 = pool.acquire().unwrap();
    assert_eq!(*g2, 15);
}

#[test]
fn pool_multiple_acquire_drop_cycles_preserve_all_items() {
    let pool = Pool::new(vec![1, 2, 3]);
    let g1 = pool.acquire().unwrap(); // pops 3
    let g2 = pool.acquire().unwrap(); // pops 2
    drop(g1); // pushes 3 back -> [1, 3]

    let g3 = pool.acquire().unwrap(); // pops 3
    assert_eq!(*g3, 3);
    assert_eq!(pool.available(), 1); // [1]

    drop(g2); // pushes 2 back -> [1, 2]
    drop(g3); // pushes 3 back -> [1, 2, 3]
    assert_eq!(pool.available(), 3);
}

#[test]
fn pool_works_with_non_copy_types() {
    let pool = Pool::new(vec![String::from("a"), String::from("b")]);
    let mut g = pool.acquire().unwrap();
    assert_eq!(*g, "b");
    g.push_str("!");
    assert_eq!(*g, "b!");
    drop(g);
    assert_eq!(pool.available(), 2);

    let g2 = pool.acquire().unwrap();
    assert_eq!(*g2, "b!");
}
