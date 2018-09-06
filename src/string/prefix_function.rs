// https://neerc.ifmo.ru/wiki/index.php?title=Префикс-функция

#[allow(dead_code)]
pub fn build_prefix_function_naive(text: &[u8]) -> Vec<usize> {
    let mut border_lengths = vec![0; text.len()];

    for i in 0..text.len() {
        for j in 0..i {
            if text[0..j + 1] == text[i - j..i + 1] {
                border_lengths[i] = j + 1;
            }
        }
    }
    border_lengths
}

#[allow(dead_code)]
pub fn build_prefix_function_effective(text: &[u8]) -> Vec<usize> {
    let mut border_lengths = vec![0; text.len()];

    for i in 1..text.len() {
        let mut k = border_lengths[i - 1];
        while k > 0 && text[i] != text[k] {
            k = border_lengths[k - 1];
        }
        if text[i] == text[k] { k += 1; }
        border_lengths[i] = k;
    }
    border_lengths
}

