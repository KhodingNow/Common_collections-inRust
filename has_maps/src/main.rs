fn main() {

    // The Hasher Trait

    use std::collections::HashMap;

    // Default HashMap with SipHash

    let mut map: HashMap<String, i32> = HashMap::new();

    // Custom hasher (e.g, faster but less secure)
    
    use std::hash::BuildHasherDefault;
    use twox_hash::XxHash64;
    type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;

// Core Methods and Usage

// 1. Inserti0n and Lookup

    let mut scores = HashMap::new();

    // Inserts key-value pairs

    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 90);

    // Get a value (returns Option<V>)
    let alice_score = scores.get("Alice"); // Some(&100)

    // Check for existance 
    if scores.contains_key("Bob") {/*....*/}

   //  2. The 'Entry' API - avoids multiple lookups when checking for existing keys

   // Insert or update a value

   scores.entry(String::from("Alice"))
   .and_modify(|v| *v += 10)
   .or_insert(50);

   // Iteration

   for (key, value) in &scores {
    printlin!(" {}: {}", key, value);
   }

   // Merging and Updating

   let mut map1 = HashMap::from([("a, 1"), ("b", 2)]);
   let mut map2 = HashMap::from([("b, 3"), ("c", 4)]);

   map1.extend(map2);  // "b" in map1 is overwritten with 3

   // Ownership and Borrowing EXAMPLE of HashMap in Rust

   let key = String::from("key");
    let value = vec![1, 2, 3];
    let mut map = HashMap::new();
    map.insert(key, value); // Ownership transferred to map 
                            // 'key' and 'value' are no longer accessible from here 


    // 3. Concurrency and Thread Safety

        // - HashMap is not thread-safe by default. For concurrent access:
        // - Use Arc<Mutex<HashMap<K, V>>> or DashMap (from the dashmap crate) for lock-free concurrency.


    // Real-world application:

    //Caching / Memoization

    fn fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        match cache.get(&n) {
            Some(&result) => result.
            None => {
                let result = match n {
                    0 | 1 => 1,
                    _ => fibonacci(n-1, cache) + fibonacci(n-2, cache),
                };
                cache.insert(n, result);
                result
            }
        }
    }

    // Counting word frequencies:

    let text = "Hello world, hello Rust world";
        let mu word_counts = HashMap::new();
        for word in text.split_whitespace() {
            *word_counts.entry(word).or_insert(0) += 1;

        }
 
        // Why HashMaps matter
        // - ubiquity use in almost every Rust program for efficient data management
        // - Memory Safety Rust's ownership model prevents common bugs( e.g, dangling pointers in C++ hash tables)
        // - Performance: Combines low-level safety, making it ideal for systems programming
        // - Ecosystems integration: Libraries like 'serde' and 'diesel' rely on hash maps for JSON/DB interactions.

        // Master Hashmaps - write efficient, safe, and idiomatic Rust code.
}
