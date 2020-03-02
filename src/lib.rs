pub fn main() {
    unimplemented!()
}

#[derive(Clone)]
pub struct Item {
    key: String,
    value: String,
}

impl Item {
    fn new(key: String, value: String) -> Self {
        Item { key, value }
    }

    fn from_strs(key: &str, value: &str) -> Self {
        let key = String::from(key);
        let value = String::from(value);
        Item::new(key, value)
    }
}

pub struct HashTable {
    size: usize,
    count: usize,
    items: Vec<Option<Item>>,
    prime_a: usize,
    prime_b: usize,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        let items = vec![None; size];
        HashTable { size, count: 0, items, prime_a: 151 , prime_b: 163 }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        let new_item = Item::from_strs(key, value);

        let mut attempts = 0;
        let mut new_index = double_hash(value, self.prime_a, self.prime_b, self.size, attempts);
        let mut curr_item = &self.items[new_index];

        while let Some(_) = curr_item {
            attempts += 1;
            new_index = double_hash(value, self.prime_a, self.prime_b, self.size, attempts);
            curr_item = &self.items[new_index];
        }
        self.items[new_index] = Some(new_item);
        self.count += 1;
    }

    fn search(&self, key: &str) -> String {
        unimplemented!()
    }

    fn delete(&self, key: &str) {
        unimplemented!()
    }
}

fn double_hash(value: &str, prime_a: usize, prime_b: usize, num_buckets: usize, attempts: usize) -> usize {
    (hash(value, prime_a, num_buckets) + attempts * (hash(value, prime_b, num_buckets) + 1)) % num_buckets
}

fn hash(value: &str, prime: usize, num_buckets: usize) -> usize {
    let mut hashed = 0;
    for (idx, chr) in value.chars().enumerate() {
        hashed += (prime.pow((value.len() - (idx + 1)) as u32)) * (chr as usize)
    }
    hashed %= num_buckets;
    hashed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let a = hash("cat", 151, 53);
        let b = hash("cat", 163, 53);
        assert_eq!(a, 5);
        assert_eq!(b, 21);
    }
}
