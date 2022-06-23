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
    assert_eq!(histogram(&input), expected)
}

#[test]
fn histogram_empty() {
    let input = vec![];
    let expected = IndexMap::new();
    assert_eq!(histogram(&input), expected)
}

#[test]
fn init_no_ties() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let input = histogram(&input);
    let expected = vec![
        Tree::Leaf(4, 4),
        Tree::Leaf(3, 3),
        Tree::Leaf(2, 2),
        Tree::Leaf(1, 1),
    ];
    assert_eq!(leaves(input), expected)
}

#[test]
fn init_with_ties() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4];
    let input = histogram(&input);
    let expected = vec![
        Tree::Leaf(3, 3),
        Tree::Leaf(4, 3),
        Tree::Leaf(2, 2),
        Tree::Leaf(1, 1),
    ];
    assert_eq!(leaves(input), expected)
}

#[test]
fn init_empty() {
    let input = vec![];
    let input = histogram(&input);
    let expected = vec![];
    assert_eq!(leaves(input), expected)
}

#[test]
fn tree_four() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let input = histogram(&input);
    let mut input = leaves(input);
    let l_4 = Tree::Leaf(4, 4);
    let l_3 = Tree::Leaf(3, 3);
    let l_2 = Tree::Leaf(2, 2);
    let l_1 = Tree::Leaf(1, 1);
    let n_1 = Tree::Node(Box::from(l_1), Box::from(l_2), 3);
    let n_2 = Tree::Node(Box::from(l_3), Box::from(n_1), 6);
    let n_3 = Tree::Node(Box::from(l_4), Box::from(n_2), 10);

    assert_eq!(huff_tree(&mut input), Some(n_3))
}

#[test]
fn tree_single() {
    let input = vec![4, 4, 4, 4];
    let input = histogram(&input);
    let mut input = leaves(input);
    let l_4 = Tree::Leaf(4, 4);

    assert_eq!(huff_tree(&mut input), Some(l_4))
}

#[test]
fn tree_double() {
    let input = vec![4, 4, 4, 4, 1];
    let input = histogram(&input);
    let mut input = leaves(input);
    let l_4 = Tree::Leaf(4, 4);
    let l_1 = Tree::Leaf(1, 1);
    let n_1 = Tree::Node(Box::from(l_1), Box::from(l_4), 5);

    assert_eq!(huff_tree(&mut input), Some(n_1))
}

#[test]
fn tree_triple() {
    let input = vec![4, 4, 4, 4, 3, 3, 3, 1];
    let input = histogram(&input);
    let mut input = leaves(input);
    let l_4 = Tree::Leaf(4, 4);
    let l_3 = Tree::Leaf(3, 3);
    let l_1 = Tree::Leaf(1, 1);
    let n_1 = Tree::Node(Box::from(l_1), Box::from(l_3), 4);
    let n_2 = Tree::Node(Box::from(l_4), Box::from(n_1), 8);

    assert_eq!(huff_tree(&mut input), Some(n_2))
}

#[test]
fn tree_empty() {
    let input = vec![];
    let input = histogram(&input);
    let mut input = leaves(input);

    assert_eq!(huff_tree(&mut input), None)
}

#[test]
fn book_single() {
    let input = vec![1];
    let input = histogram(&input);
    let mut input = leaves(input);
    let input = huff_tree(&mut input).unwrap();
    let mut expected = IndexMap::new();
    expected.insert(1, 0);

    assert_eq!(code_lengths(input), expected)
}
#[test]
fn book_two() {
    let input = vec![1, 2, 2];
    let input = histogram(&input);
    let mut input = leaves(input);
    let input = huff_tree(&mut input).unwrap();
    let mut expected = IndexMap::new();
    expected.insert(2, 1);
    expected.insert(1, 1);

    assert_eq!(code_lengths(input), expected)
}

#[test]
fn book_four() {
    let input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let input = histogram(&input);
    let mut input = leaves(input);
    let input = huff_tree(&mut input).unwrap();
    let mut expected = IndexMap::new();
    expected.insert(4, 1);
    expected.insert(3, 2);
    expected.insert(1, 3);
    expected.insert(2, 3);

    assert_eq!(code_lengths(input), expected)
}

#[test]
fn book_canonical() {
    let l1 = Tree::Leaf(4, 6);
    let l2 = Tree::Leaf(2, 7);
    let l3 = Tree::Leaf(3, 4);
    let l4 = Tree::Leaf(1, 8);
    let l5 = Tree::Leaf(5, 5);
    let n1 = Tree::Node(Box::new(l3), Box::new(l5), 9);
    let input = Tree::Node(Box::new(l1), Box::new(l2), 13);
    let n1 = Tree::Node(Box::new(l4), Box::new(n1), 9);
    let input = Tree::Node(Box::new(input), Box::new(n1), 13);
    let mut expected = IndexMap::new();
    expected.insert(1, String::from("00"));
    expected.insert(2, String::from("01"));
    expected.insert(4, String::from("10"));
    expected.insert(3, String::from("110"));
    expected.insert(5, String::from("111"));
    let res = code_lengths(input);
    let res = enc_vector(&res).unwrap();
    let res = canonical_book(res);

    assert_eq!(res, expected)
}

#[test]
fn book_encode() {
    let l1 = Tree::Leaf(4, 6);
    let l2 = Tree::Leaf(2, 7);
    let l3 = Tree::Leaf(3, 4);
    let l4 = Tree::Leaf(1, 8);
    let l5 = Tree::Leaf(5, 5);
    let n1 = Tree::Node(Box::new(l3), Box::new(l5), 9);
    let input = Tree::Node(Box::new(l1), Box::new(l2), 13);
    let n1 = Tree::Node(Box::new(l4), Box::new(n1), 9);
    let input = Tree::Node(Box::new(input), Box::new(n1), 13);
    let input = code_lengths(input);
    let input = enc_vector(&input).unwrap();
    let input = canonical_book(input);
    let expected = vec![0, 3, 2, 1, 2, 4, 3, 5];

    assert_eq!(dump_used(input), expected)
}
