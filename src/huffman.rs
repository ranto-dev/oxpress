/**
 * algorithme de huffman
 */
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanTree {
    pub codes: HashMap<u8, Vec<bool>>,
}

#[derive(Eq)]
struct Node {
    freq: usize,
    byte: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

pub fn compress(data: &[u8]) -> (Vec<u8>, HuffmanTree) {
    let mut freq = HashMap::new();

    for &b in data {
        *freq.entry(b).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();

    for (byte, f) in freq {
        heap.push(Node {
            freq: f,
            byte: Some(byte),
            left: None,
            right: None,
        });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        heap.push(Node {
            freq: left.freq + right.freq,
            byte: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
    }

    let root = heap.pop().unwrap();

    let mut codes = HashMap::new();
    build_codes(&root, Vec::new(), &mut codes);

    let mut encoded_bits = Vec::new();
    for &b in data {
        encoded_bits.extend(&codes[&b]);
    }

    let mut bytes = Vec::new();
    let mut current = 0u8;
    let mut count = 0;

    for bit in encoded_bits {
        current <<= 1;
        if bit {
            current |= 1;
        }
        count += 1;

        if count == 8 {
            bytes.push(current);
            current = 0;
            count = 0;
        }
    }

    if count > 0 {
        current <<= 8 - count;
        bytes.push(current);
    }

    (bytes, HuffmanTree { codes })
}

fn build_codes(node: &Node, prefix: Vec<bool>, map: &mut HashMap<u8, Vec<bool>>) {
    if let Some(byte) = node.byte {
        map.insert(byte, prefix);
    } else {
        if let Some(ref left) = node.left {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            build_codes(left, left_prefix, map);
        }
        if let Some(ref right) = node.right {
            let mut right_prefix = prefix;
            right_prefix.push(true);
            build_codes(right, right_prefix, map);
        }
    }
}

pub fn decompress(data: &[u8], tree: &HuffmanTree) -> Vec<u8> {
    let mut reverse = HashMap::new();
    for (k, v) in &tree.codes {
        reverse.insert(v.clone(), *k);
    }

    let mut bits = Vec::new();
    for byte in data {
        for i in (0..8).rev() {
            bits.push(byte & (1 << i) != 0);
        }
    }

    let mut decoded = Vec::new();
    let mut buffer = Vec::new();

    for bit in bits {
        buffer.push(bit);
        if let Some(byte) = reverse.get(&buffer) {
            decoded.push(*byte);
            buffer.clear();
        }
    }

    decoded
}
