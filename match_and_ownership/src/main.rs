fn main() {
    let opt: Option<String> = Some(String::from("Hello Workd!"));

    // match opt {
    match &opt {
        // this will compile
        // Some(_) => println!("Some!"),
        // but not this (_ replaced by s)
        // to fix it, use a reference to opt in the match
        Some(s) => println!("Some {}", s),
        None => println!("None!")
    }

    println!("{:?}", opt);
}
