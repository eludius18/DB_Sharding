use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const NUMBER_OF_SHARDS: usize = 4;

struct Shard {
    data: HashMap<String, String>, // Simulating a simple key-value store
}

impl Shard {
    fn new() -> Self {
        Shard {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn update(&mut self, key: String, value: String) -> Option<String> {
        self.data.insert(key, value)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

struct ShardedDatabase {
    shards: Vec<Shard>,
}

impl ShardedDatabase {
    fn new() -> Self {
        let mut shards = Vec::with_capacity(NUMBER_OF_SHARDS);
        for _ in 0..NUMBER_OF_SHARDS {
            shards.push(Shard::new());
        }
        ShardedDatabase { shards }
    }

    fn determine_shard<T: Hash +?Sized>(&self, key: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % NUMBER_OF_SHARDS
    }

    fn insert(&mut self, key: String, value: String) {
        let shard_id = self.determine_shard(&key);
        self.shards[shard_id].insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        let shard_id = self.determine_shard(key);
        self.shards[shard_id].get(key)
    }

    fn update(&mut self, key: String, value: String) -> Option<String> {
        let shard_id = self.determine_shard(&key);
        self.shards[shard_id].update(key, value)
    }
    
    fn delete(&mut self, key: &str) -> Option<String> {
        let shard_id = self.determine_shard(key);
        self.shards[shard_id].delete(key)
    }
}

fn main() {
    let mut db = ShardedDatabase::new();
    
    // Insert some data
    db.insert("user1".to_string(), "Alice".to_string());
    db.insert("user2".to_string(), "Bob".to_string());
    
    // Retrieve and print a value
    if let Some(name) = db.get("user1") {
        println!("Found: {}", name);
    }
    
    // Update a value
    db.update("user1".to_string(), "Alicia".to_string());

    // Delete a value
    db.delete("user2");
    
    // Try to retrieve a deleted value
    if db.get("user2").is_none() {
        println!("User2 deleted successfully");
    }
}