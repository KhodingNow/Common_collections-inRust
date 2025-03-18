Hash maps are data structures that store KEY-VALUE pairs, allowing efficient lookups, inserts, and deletes. But in Rust, there are specific implementations and considerations, like standard HashMap from the std::collections module.

How do Hashmaps work under the hood - hash functions, buckets, collisions resolutions. Rust a hashbrown crate as the underlying implementation.HasMaps provide a O(1) average time complexityfor operations, which is crucial for performance sensitive apps. Also, in Rust's ownership and borrowing system, how keys and values are handled - ownership vs borrowing - what are the implications of using types that don't implement Copy.

Potential pitfall:
- Handling collisions, what the importance of a good hash function to avoidperformance degradation, what about the fact that in Rust hashMaps are not ordered?
Thread safety - if you are working in a concurrent environment, do you need to use Arc or Mutex with hash maps?

HashMaps are important / fundamental in many algorithyms and applications. In Rust, understanding their specifics helps write efficient and safe code. For example, knowing that inserting into a hashMap can re-allocate and invalidate references, so you have to manage ownership correctly.

Also, how does Rust handle securityaspects like HashDoS attacks, where an attacker cause exxcessive collisions. The default hasher in Rust (SipHash) is slower, while allowing the option to switchto a faster hasher if needed.

What is a HashMap?

A hashmap (hash table) is a data structure that stores key-value pairs, enabling efficient insertion, lookup, deletion of vlues by their keys.
In Rust. the primary implementation is std::colletions::HashMap, which uses a hashing algorithm to make keys to their corresponding values in memory.

Key Characteristics:

- Key-Value storage: Store data as pair (e.g, (user_id: u32, username: String)).
- Average O(1) Complexity: Mostoperations (insert, get, remove) are constant-time on average.
- Unordered: Keys are not stored in a sorted order (use BTreeMap for ordered key-value storage).
- Ownership: Keys and values are owned by the hash map unless borrowed explicitly.

How HashMaps work in Rust

* Under the Hood

= Hashing: A hash function converts keys into indices (buckets) where values are stored.
= Collision Resolution: Rust uses open addressing with Robin Hood hashing (via the hashbrown crate) to handle collisions (multiple keys hashing to the same bucket)

= Load factor: The hash map automatically resizes (rehashes) when the number of elements exceed a capacity threshold, balancing memory and performance.

Hashing in Rust

- The Hash Trait: Types used as keys must implement Hash (e.g, String, i32)
- The Hasher Trait: Defines how keys are hashed. Rust's default hasher is SipHas 1-3, which is resilient to HashDos attacks but slower than non-cryptographic hashers.

    use std::collections::HashMap;

    // Default Hasher Map with SipHash
    let mut map: HashMap<String, i32> = HashMap::new();

    // Custom hasher (e.g, faster but less secure)
    use std::hash::BuildHasherDefault;
    use twox_hash::XxHash64;
    type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;

Core Methods and Usage

1. Inserti0n and Lookup

    let mut scores = HashMap::new();

    // Inserts key-value pairs

    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 90);

    // Get a value (returns Option<V>)
    let alice_score = scores.get("Alice); // Some(&100)

    // Check for existance 
    if scores.contains_key("Bob") {/*....*/}

    2. The 'Entry' API

    Avoids multiple lookups when checking for existing keys:

        // Insert orn update a value
        scores.entry(String::from("Alice"))
        .and_modify(|v| *v += 10)
        .or_insert(50);