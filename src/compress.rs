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

pub fn compress(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read(conf.filename)?;
    let hist = histogram(&content);
    let tree = huff_tree(&mut leaves(hist));
    let tree = opt_result(tree, "Could not build a tree")?;
    let len_book = code_lengths(tree);
    let encode_vector = opt_result(
        enc_vector(&len_book),
        "Failed to create a canonical code book",
    )?;
    let book = canonical_book(encode_vector);
    let mut encoding = Vec::new();
    let mut buffer = String::new();
    for byte in content {
        buffer = buffer + opt_result(book.get(&byte), "Couldn't find the encoding for a byte")?;
        while buffer.len() > 7 {
            let byte_string = buffer.drain(..8);
            let byte = u8::from_str_radix(byte_string.as_str(), 2)?;
            encoding.push(byte);
        }
    }
    let leftover = buffer.len();
    if leftover != 0 {
        buffer = buffer + &"1".repeat(8 - leftover);
        let byte = u8::from_str_radix(&buffer, 2)?;
        encoding.push(byte);
    }
    let mut header = create_header(MetaData {
        leftover: leftover as u8,
        book: book,
        code_lenghts: len_book,
    });
    header.append(&mut encoding);
    fs::write(format!("{}.mhf", conf.filename), header)?;

    return Ok(());
}

pub struct MetaData {
    leftover: u8,
    book: IndexMap<u8, String>,
    code_lenghts: IndexMap<u8, usize>,
}

pub fn create_header(metadata: MetaData) -> Vec<u8> {
    let top_3 = metadata.leftover << 5;
    let mut header = Vec::new();
    let l = metadata.book.len() as u8;
    if l > (u8::MAX / 2) {
        header.push(top_3 | 0b0001_0000);
        header.append(&mut dump_all(metadata.code_lenghts));
    } else {
        header.push(top_3);
        header.push(l);
        header.append(&mut dump_used(metadata.book))
    };
    header
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

pub fn enc_vector(len_book: &IndexMap<u8, usize>) -> Option<Vec<Vec<u8>>> {
    let mut longest = 0;
    for &len in len_book.values() {
        longest = longest.max(len);
    }
    let mut encode_vector: Vec<Vec<u8>> = vec![Vec::new(); longest];
    for (byte, enc_len) in len_book.iter() {
        encode_vector[*enc_len - 1].push(*byte);
    }
    for chars_of_len in encode_vector.iter_mut() {
        chars_of_len.sort();
    }
    Some(encode_vector)
}

pub fn canonical_book(encode_vector: Vec<Vec<u8>>) -> IndexMap<u8, String> {
    let mut code_book = IndexMap::new();
    let mut global_byte: u32 = 0;
    for (len, chars) in encode_vector.iter().enumerate() {
        match chars {
            empty if empty.is_empty() => {}
            non_empty => {
                for &c in non_empty {
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
    code_book
}

pub fn dump_all(len_book: IndexMap<u8, usize>) -> Vec<u8> {
    let mut encoding = Vec::new();
    for c in 0..=u8::MAX {
        let enc_len = *len_book.get(&c).unwrap_or_else(|| &0);
        encoding.push(enc_len as u8);
    }
    encoding
}

pub fn dump_used(book: IndexMap<u8, String>) -> Vec<u8> {
    let mut encoding = Vec::new();
    let mut enc_i = 1;
    let mut enc_n = 0;
    for code in book.values() {
        let curr_len = code.len();
        while enc_i < curr_len {
            encoding.push(enc_n);
            enc_i += 1;
            enc_n = 0;
        }
        enc_n += 1;
    }
    encoding.push(enc_n);
    for c in book.keys() {
        encoding.push(*c);
    }
    encoding
}
