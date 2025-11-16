fn main() {
    let sentence = String::from("Rust is fast and memory safe 😃");

    let first = first_word(&sentence);
    let second = second_word(&sentence);
    let longest = longest_word(&sentence);
    let total_words = word_count(&sentence);
    let nth_word = nth_word(&sentence, 5);

    println!("The first word is {first}");
    println!("The second word is {second}");
    println!("The longest word is {longest}");
    println!("The total number of words is {total_words}");
    println!("The nth word in the sentence is {nth_word}");
}

fn first_word(s: &String) -> &str {             // Getting the first word
    let bytes = s.as_bytes();           // Converting it all into bytes so we can locate the space "b' '"

    for (i, &item) in bytes.iter().enumerate() {           // Get us the (index, byte size) in a tuple
                                                                    // We use a reference to the bytes (item) becuase the String is referenced and it returns a &u8
        if item == b' ' {                    //  If bytes == space, break the loop and return the slice from the first letter up until the space
            return &s[0..i];
        }
    }
    &s[..]                                  // Return this result so it can be used in other parts, nott just this function. e.g. we can call it up in main fn
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_space = None;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_space.is_none() {
                first_space = Some(i);
            } else {
                return &s[first_space.unwrap() + 1..i];
            }
        }
    }
    ""
}

fn longest_word(s: &String) -> &str {
    let mut longest = "";               // We'll use this to compare the longest words
    let mut current_start = 0;         // For tracking the characters in the word

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let word = &s[current_start..i];            
            if word.len() > longest.len() {
                longest = word;
            }

            current_start = i + 1;
        }
    }

    let last = &s[current_start..];
    if last.len() > longest.len() {
        longest = last;
    }

    longest
}

fn word_count(s: &String) -> usize {
    let bytes = s.as_bytes();

    let mut count = 0;
    let mut in_word = false;

    for &item in bytes {
        if item != b' ' {
            if !in_word {
                count += 1;
                in_word = true;
            } 
        } else {
            in_word = false;
        }
    }

    count
}

fn nth_word(s: &String, n: usize) -> &str {
    let bytes = s.as_bytes();                   // convert String into bytes for easier parsing

    let mut word_count = 0;
    let mut start = 0;
    let mut in_word = false;

    for (i, &item) in bytes.iter().enumerate() {                // parse over the bytes and indexes, we're trying to find the space which is 32 bytes

        if item != b' ' {                       // if we don't get a 'space' and we're in a word, add 1 to the word_count
            if !in_word {

                word_count += 1;
                start = i;
                in_word = true;
            }

        } else {
            if in_word && word_count == n {         // when we get to the end of the letters or another space section in the String...

                return &s[start..i];                // return from the first letter to the last letter as a complete one
            }

            in_word = false;
        }
    }

    if in_word && word_count == n {
        return &s[start..s.len()];
    }
    ""
}