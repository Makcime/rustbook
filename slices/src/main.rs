fn bad_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }


    s.len() // return the len of the string if no space found
}

fn _first_word(s: &String) -> &str {
    let bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[0..i];
        }
    }

    &s[..]
}

// Improving the first_word function by using a string slice for the type of the s parameter
// better signature for the "first_word" function
// it allows to use both String type and string literal
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("Hello workd");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we
    // could meaningfully ise the value 5 with. word is now totally invalid!

    // let use String slieces instead

    let mut s = String::from("Hello workd");

    let hello = &s[0..5];
    let world = &s[6..11];


    // we get a compile-time error if we try to use work, now that we fixed the "first_word"
    // funciton

    // println!("the first word is : {}", word);


    let my_string = String::from("Hello World");

    // first_word works on slieces of 'String's, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // 'first_word' also workd on references to ' String's, which are equivvalent
    // to whole slieces of 'String's
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // 'first_word' works on slice of string literal, whether partial or whole

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice synthax !
    let word = first_word(my_string_literal);


    // Other type of slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
