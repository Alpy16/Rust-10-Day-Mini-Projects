fn main() {
    //Slice ParserTake a sentence and return a reference (&str) to the third word, iterating it byte by byte.
    println!("Please input a minimum 3 word sentence:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read");
    let cleaned_input = input.trim();
    match third_word(cleaned_input) {
        Some(word) => println!("Found it: {}", word),
        None => println!("The sentence was too short!"),
    }
}

fn third_word(input: &str) -> Option<&str> {
    let mut count = 0;
    let mut start = 0;

    for (i, &item) in input.as_bytes().iter().enumerate() {
        // using iter allows us to iterate every item (letter), but enumarate allows us to label every item with a number
        if item == b' ' {
            count += 1;

            if count == 2 {
                // Bookmark the start (index after the 2nd space)
                start = i + 1;
            } else if count == 3 {
                // We found the end! Return the slice from start to current index
                return Some(&input[start..i]);
            }
        }
    }

    // After the loop, check if we found exactly two spaces
    if count == 2 && input.len() > start {
        Some(&input[start..])
    } else {
        None
    }
}
