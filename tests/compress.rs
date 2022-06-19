use std::f32::consts::E;

use indexmap::IndexMap;
use multirust::compress::*;

#[test]
fn histogram_valid() {
    let input = vec![1, 2, 3, 3, 3, 4];
    let mut expected = IndexMap::new();
    expected.insert(1, 1);
    expected.insert(2, 1);
    expected.insert(3, 3);
    expected.insert(4, 1);
    assert_eq!(create_histogram(&input), expected)
}

#[test]
fn histogram_empty() {
    let input = vec![];
    let expected = IndexMap::new();
    assert_eq!(create_histogram(&input), expected)
}

#[test]
fn init_no_ties() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let input = create_histogram(&input);
    let expected = vec![
        Tree::Leaf(4, 4),
        Tree::Leaf(3, 3),
        Tree::Leaf(2, 2),
        Tree::Leaf(1, 1),
    ];
    assert_eq!(init_leaves(input), expected)
}

#[test]
fn init_with_ties() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4];
    let input = create_histogram(&input);
    let expected = vec![
        Tree::Leaf(3, 3),
        Tree::Leaf(4, 3),
        Tree::Leaf(2, 2),
        Tree::Leaf(1, 1),
    ];
    assert_eq!(init_leaves(input), expected)
}

#[test]
fn init_empty() {
    let input = vec![];
    let input = create_histogram(&input);
    let expected = vec![];
    assert_eq!(init_leaves(input), expected)
}

#[test]
fn tree_four() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let l_4 = Tree::Leaf(4, 4);
    let l_3 = Tree::Leaf(3, 3);
    let l_2 = Tree::Leaf(2, 2);
    let l_1 = Tree::Leaf(1, 1);
    let n_1 = Tree::Node(Box::from(l_1), Box::from(l_2), 3);
    let n_2 = Tree::Node(Box::from(l_3), Box::from(n_1), 6);
    let n_3 = Tree::Node(Box::from(l_4), Box::from(n_2), 10);

    assert_eq!(build_tree(&mut input), Some(n_3))
}

#[test]
fn tree_single() {
    let input = vec![4, 4, 4, 4];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let l_4 = Tree::Leaf(4, 4);

    assert_eq!(build_tree(&mut input), Some(l_4))
}

#[test]
fn tree_double() {
    let input = vec![4, 4, 4, 4, 1];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let l_4 = Tree::Leaf(4, 4);
    let l_1 = Tree::Leaf(1, 1);
    let n_1 = Tree::Node(Box::from(l_1), Box::from(l_4), 5);

    assert_eq!(build_tree(&mut input), Some(n_1))
}

#[test]
fn tree_triple() {
    let input = vec![4, 4, 4, 4, 3, 3, 3, 1];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let l_4 = Tree::Leaf(4, 4);
    let l_3 = Tree::Leaf(3, 3);
    let l_1 = Tree::Leaf(1, 1);
    let n_1 = Tree::Node(Box::from(l_1), Box::from(l_3), 4);
    let n_2 = Tree::Node(Box::from(l_4), Box::from(n_1), 8);

    assert_eq!(build_tree(&mut input), Some(n_2))
}

#[test]
fn tree_empty() {
    let input = vec![];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);

    assert_eq!(build_tree(&mut input), None)
}

#[test]
fn lookup_single() {
    let input = vec![1];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let input = build_tree(&mut input).unwrap();
    let mut expected = IndexMap::new();
    expected.insert(1, "");

    assert_eq!(make_lookup(input), expected)
}
#[test]
fn lookup_two() {
    let input = vec![1, 2, 2];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let input = build_tree(&mut input).unwrap();
    let mut expected = IndexMap::new();
    expected.insert(2, "1");
    expected.insert(1, "0");

    assert_eq!(make_lookup(input), expected)
}

#[test]
fn lookup_four() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let input = create_histogram(&input);
    let mut input = init_leaves(input);
    let input = build_tree(&mut input).unwrap();
    let mut expected = IndexMap::new();
    expected.insert(4, "0");
    expected.insert(3, "10");
    expected.insert(1, "110");
    expected.insert(2, "111");

    assert_eq!(make_lookup(input), expected)
}
