// https://neerc.ifmo.ru/wiki/index.php?title=Алгоритм_Кнута-Морриса-Пратта

use string::prefix_function::*;

pub struct KMPSearch {
    prefix_values: Vec<usize>,
}

impl KMPSearch {
    pub fn new() -> Self {
        KMPSearch{ prefix_values: Vec::<usize>::new() }
    }

    pub fn build(&mut self, text: &[u8], pattern: &[u8], delimiter: u8) {
        let mut buffer = Vec::<u8>::with_capacity(text.len() + pattern.len() + 1);
        buffer.extend_from_slice(pattern);
        buffer.push(delimiter);
        buffer.extend_from_slice(text);

        self.prefix_values = build_prefix_function_effective(&buffer);
    }

    pub fn find_first(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        assert_eq!(text.len() + pattern.len() + 1, self.prefix_values.len());

        for i in 0..text.len() {
            if self.prefix_values[i + pattern.len() + 1] == pattern.len() { return Some(i + 1 - pattern.len()); }
        }
        None
    }
}
