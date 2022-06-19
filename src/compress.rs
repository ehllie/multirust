use indexmap::IndexMap;
use std::{error::Error, fs};
pub struct Config<'a> {
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Compress tool requires a filename argument");
        };
        let filename = &args[2];
        Ok(Config { filename })
    }
}

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read(filename)?;

    return Ok(());
}

#[derive(Debug)]
pub enum Tree {
    Leaf(u8, u32),
    Node(Box<Tree>, Box<Tree>, u32),
}

impl Tree {
    fn val(&self) -> &u32 {
        match self {
            Tree::Leaf(_, v) => v,
            Tree::Node(_, _, v) => v,
        }
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lval = self.val();
        let rval = other.val();
        Some(lval.cmp(rval))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

pub fn create_histogram(content: &[u8]) -> IndexMap<u8, u32> {
    let mut hist = IndexMap::new();
    for byte in content {
        let val;
        match hist.get(byte) {
            Some(old_hist) => val = old_hist + 1,
            None => val = 1,
        };
        hist.insert(*byte, val);
    }
    hist
}

pub fn init_leaves<'a>(histogram: IndexMap<u8, u32>) -> Vec<Tree> {
    let mut sorted = Vec::new();
    for (k, v) in histogram.sorted_by(|_, l, _, r| r.cmp(l)) {
        sorted.push(Tree::Leaf(k, v));
    }
    sorted
}

pub fn build_tree(leaves: &mut Vec<Tree>) -> Option<Tree> {
    let first = leaves.pop()?;
    let second;
    match leaves.pop() {
        None => {
            return Some(first);
        }
        Some(other) => second = other,
    }
    let v = first.val() + second.val();
    let new_node = Tree::Node(Box::from(first), Box::from(second), v);
    for (i, node) in leaves.iter().enumerate() {
        if node.val() <= new_node.val() {
            leaves.insert(i, new_node);
            return build_tree(leaves);
        };
    }
    leaves.push(new_node);
    build_tree(leaves)
}

pub fn make_lookup(huff_tree: Tree) -> IndexMap<u8, String> {
    let mut lookup = IndexMap::new();
    // println!("{:?}", huff_tree);
    match huff_tree {
        Tree::Leaf(k, _) => {
            lookup.insert(k, String::new());
        }
        Tree::Node(left, right, _) => {
            let left = make_lookup(*left);
            let right = make_lookup(*right);
            for (c, enc) in left.iter() {
                let enc = format!("0{}", enc);
                // println!("{}: \"{}\"", c, enc);
                lookup.insert(*c, enc);
            }
            for (c, enc) in right.iter() {
                let enc = format!("1{}", enc);
                // println!("{}: \"{}\"", c, enc);
                lookup.insert(*c, enc);
            }
        }
    }
    lookup
}
