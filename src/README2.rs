fn main() {
    // Code Snippets examples for String and '&str'

    let s = String::from ("hello"); // UTF-8 bytes: [104, 101, 108, 111]
    let emoji = "😊"; // UTF-8 bytes: [240, 159, 152, 138]

    // Key implemantations of UTF-8 in Rust

    // A. You can't index by Character Position

    // Direct indexing like s[0] is disallowed because character (Unicode scalar values) can span multiple bytes:

    let s = "FXCKSIANg"; // a complex charecter from a non English language
    // s[0] would compile to a byte (e.g, 224), not a character.
    // Instead, use iterators to work with characters:

    for c in s.chars() { // iterate over Unicode scalar values
        printlin!("{}", c);

    }

    // B. String Lengths are Tricky
    
    // - s.len() returns the byte count, not character count:
    
    printlin!("{}", "👍".len()); // 4 bytes, not 1 character 
    
    // Use s.chars().count() for character count (but its O(n)).

    // Invalid UTF-8 is Rejected

    // Rust enforces valid UTF-8 at compile time for String/&str. If you have a raw bytes (e.g, from a file or network), you must handle errors:
    
    let bytes = vec![0xFF, 0x00]; // invalid UTF-8
    let s = String::from_utf8(bytes); // Returns Result<String, FromUtf8Error>

    //For lossy conversion (replace invlid sequences with ?):

    let s = String::from_utf8_lossy(&[0xFF, 0x00]); // '?\x00'

    // Why UTF-8 in Rust?

    //1 Compatibility - UTF-8 is a web standard and dominates modern systems
    // 2 Memory Efficiency - Saves space for ASCII-heavy text (e.g JSON, HTML).
    // Safety: Prevents common bugs like buffer overflows from misaligned character boundaries
    // Interoperability : works seamlessly with C libraries (which often expect UTF-8).

    // 5. Working with UFT-8 Strings
    
    // A. Use byte ranges, but ensure they align to UTF-8 boundaries:

    let s = "hello😂";
    let sub = &s[0..5]; // 'hello' (valid)
    // let sub = &s[0..3]; Panicks: 'hel' is valid, but slicing mid-character isn't

    // B. Handling Grapheme Clusters

    // Some 'characters' are combinations of Unicode scalar values (e' = e + ')
    
    [dependencies]
    unicode-segmentation = "1.10.0"

    use unicode_segmentation::UnicodeSegmentation;

    for g in "e".grapheme(true) { // Treat 'e' as a single grapheme
        printlin!("{}", g); // "e"

    }

    // Convering to/from Bytes 
    // To handle raw bytes (e.g, network)

    // Valid UTF-8 -> String

    let valid_bytes = vec![104, 108, 108, 111]; // 'hello'
    let s = String::from_utf8(valid_bytes).unwrap();

    // Invalid UTF-8 ->Lossy String

    let invalid_bytes = vec![255];
    let s String::from_utf8_lossy(&invalid_bytes); '?'
    
}
