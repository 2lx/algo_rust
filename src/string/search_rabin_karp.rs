// https://neerc.ifmo.ru/wiki/index.php?title=Поиск_подстроки_в_строке_с_использованием_хеширования._Алгоритм_Рабина-Карпа

pub struct RKSearch {
    base: usize,
    prime: usize,
}

impl RKSearch {
    pub fn new(base: usize, prime: usize) -> Self {
        RKSearch{ base, prime }
    }

    fn set_power(&self, power: usize) -> usize {
        (0..power).fold(1usize, |p, _| (p * self.base) % self.prime)
    }

    fn hash(&self, strin: &[u8]) -> usize {
        strin.iter()
            .map(|&c| c as usize)
            .fold(0usize, |p, u| (self.base * p + u) % self.prime)
    }

    pub fn find_first(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        if text.len() < pattern.len() { return None; }
        let base_powered = self.set_power(pattern.len() - 1);
        let pattern_hash = self.hash(pattern);
        let mut text_hash = self.hash(&text[..pattern.len()]);

        for i in 0..text.len() - pattern.len() {
            if pattern_hash == text_hash && &text[i..pattern.len() + i] == pattern {
                return Some(i);
            }
            let hash_term = text_hash + self.prime - (text[i] as usize * base_powered) % self.prime;
            text_hash = (self.base * hash_term + text[pattern.len() + i] as usize) % self.prime;
        }
        None
    }
}
