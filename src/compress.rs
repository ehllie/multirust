use indexmap::IndexMap;
use std::{cmp::Ordering, error::Error, fs};
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

// pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
//     let content = fs::read(filename)?;
//     let hist = histogram(&content);
//     let tree = huff_tree(&mut leaves(hist));
//     let tree = opt_result(tree, "Could not build a tree")?;
//     let lookup = code_book(tree);
//     let mut encoding = encode_book(&lookup);
//     let mut buffer = String::new();
//     for byte in content {
//         buffer = buffer + opt_result(lookup.get(&byte), "Couldn't find the encoding for a byte")?;
//         if buffer.len() > 7 {
//             let byte = opt_result(
//                 enc_buffer_to_bytes(&mut buffer),
//                 "Could not convert buffer into a byte",
//             )?;
//             encoding.push(byte);
//         }
//     }
//     let leftover = buffer.len() as u8;
//     if leftover != 0 {
//         buffer = buffer + "1111111";
//         let byte = opt_result(
//             enc_buffer_to_bytes(&mut buffer),
//             "Could not convert buffer into a byte",
//         )?;
//         encoding.push(byte);
//     }

//     return Ok(());
// }

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

pub fn histogram(content: &[u8]) -> IndexMap<u8, u32> {
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

pub fn leaves<'a>(histogram: IndexMap<u8, u32>) -> Vec<Tree> {
    let mut sorted = Vec::new();
    for (k, v) in histogram.sorted_by(|_, l, _, r| r.cmp(l)) {
        sorted.push(Tree::Leaf(k, v));
    }
    sorted
}

pub fn huff_tree(leaves: &mut Vec<Tree>) -> Option<Tree> {
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
            return huff_tree(leaves);
        };
    }
    leaves.push(new_node);
    huff_tree(leaves)
}

fn opt_result<'a, T>(opt: Option<T>, err: &'a str) -> Result<T, &'a str> {
    match opt {
        Some(res) => Ok(res),
        None => Err(err),
    }
}

pub fn code_lengths(huff_tree: Tree) -> IndexMap<u8, usize> {
    let mut book = IndexMap::new();
    // println!("{:?}", huff_tree);
    match huff_tree {
        Tree::Leaf(k, _) => {
            book.insert(k, 0);
        }
        Tree::Node(left, right, _) => {
            let left = code_lengths(*left);
            let right = code_lengths(*right);
            for (byte, enc_len) in left.iter() {
                book.insert(*byte, *enc_len + 1);
            }
            for (byte, enc_len) in right.iter() {
                book.insert(*byte, *enc_len + 1);
            }
        }
    }
    book
}

pub fn canonical_book(len_book: IndexMap<u8, usize>) -> Option<IndexMap<u8, String>> {
    let longest = *len_book.last()?.1;
    let mut encode_vector: Vec<Vec<u8>> = vec![Vec::new(); longest];
    for (byte, enc_len) in len_book.iter() {
        encode_vector[*enc_len - 1].push(*byte);
    }
    let mut code_book = IndexMap::new();
    let mut global_byte: u8 = 0;
    for (len, chars) in encode_vector.iter_mut().enumerate() {
        match chars {
            empty if empty.is_empty() => {}
            non_empty => {
                non_empty.sort();
                for &mut c in non_empty {
                    let b_string = format!("{:b}", global_byte);
                    let pad = "0".repeat(0.max(len + 1 - b_string.len()));
                    let padded = format!("{}{}", pad, b_string);
                    code_book.insert(c, padded);
                    global_byte += 1;
                }
                global_byte <<= 1
            }
        }
    }
    Some(code_book)
}

pub fn enc_buffer_to_bytes(buffer: &mut String) -> Option<u8> {
    if buffer.len() < 8 {
        return None;
    };
    let mut byte = 0;
    for bit in buffer.drain(..8) {
        byte = byte << 1;
        if bit == '1' {
            byte += 1;
        }
    }

    Some(byte)
}

pub fn encode_book(book: &IndexMap<u8, String>) -> Vec<u8> {
    let mut encoding = Vec::new();
    for code in book.values() {
        encoding.push(code.len() as u8);
    }
    for c in book.keys() {
        encoding.push(*c);
    }
    encoding
}
