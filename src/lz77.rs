use serde::{Deserialize, Serialize};

const WINDOW_SIZE: usize = 4096;
const LOOKAHEAD_SIZE: usize = 18;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub offset: u16,
    pub length: u16,
    pub next: u8,
}

pub fn compress(input: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let start = if i > WINDOW_SIZE { i - WINDOW_SIZE } else { 0 };

        let mut best_offset = 0;
        let mut best_length = 0;

        for j in start..i {
            let mut length = 0;

            while length < LOOKAHEAD_SIZE
                && i + length < input.len()
                && input[j + length] == input[i + length]
            {
                length += 1;
            }

            if length > best_length {
                best_length = length;
                best_offset = i - j;
            }
        }

        if best_length > 0 {
            let next = if i + best_length < input.len() {
                input[i + best_length]
            } else {
                0
            };

            tokens.push(Token {
                offset: best_offset as u16,
                length: best_length as u16,
                next,
            });

            i += best_length + 1;
        } else {
            tokens.push(Token {
                offset: 0,
                length: 0,
                next: input[i],
            });

            i += 1;
        }
    }

    tokens
}

pub fn decompress(tokens: &[Token]) -> Vec<u8> {
    let mut output = Vec::new();

    for token in tokens {
        if token.offset == 0 && token.length == 0 {
            output.push(token.next);
        } else {
            let start = output.len() - token.offset as usize;

            for i in 0..token.length as usize {
                let byte = output[start + i];
                output.push(byte);
            }

            if token.next != 0 {
                output.push(token.next);
            }
        }
    }

    output
}
