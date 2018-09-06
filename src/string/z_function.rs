// https://neerc.ifmo.ru/wiki/index.php?title=Z-функция
use std::cmp::{min, max};

#[allow(dead_code)]
pub fn build_z_function_naive(text: &[u8]) -> Vec<usize> {
    let mut z_values = vec![0; text.len()];
    z_values[0] = text.len();

    for i in 1..text.len() {
        while i + z_values[i] < text.len() && text[z_values[i]] == text[i + z_values[i]] {
                z_values[i] += 1;
        }
    }
    z_values
}

#[allow(dead_code)]
pub fn build_z_function_effective(text: &[u8]) -> Vec<usize> {
    let len = text.len();
    let mut z_values = vec![0; len];
    z_values[0] = len;
    let (mut left, mut right) = (0usize, len);

    for i in 1..len {
        z_values[i] = max(0, min(right - i, z_values[i - left]));
        while i + z_values[i] < len && text[z_values[i]] == text[i + z_values[i]] {
            z_values[i] += 1;
        }
        if i + z_values[i] > right {
            left = i;
            right = i + z_values[i];
        }
    }
    z_values
}

