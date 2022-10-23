fn main() {
    let n = 7;

    if n < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    if n != 0 {
        println!("number was three");
    }

    let n = 6;

    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3 or 2");
    }


    let condition = true;
    let n = if condition {5} else {6};

    println!("The value of n is: {n}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop {
           println!("remaining = {remaining}") ;
           if remaining == 9 {
               break;
           }
           if count == 2 {
               break 'counting_up;
           }
           remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}


