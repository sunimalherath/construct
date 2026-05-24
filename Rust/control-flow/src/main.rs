fn main() {
    let num: i32 = 42;

    if num >= 0 && num < 50 {
        println!("In the correct range");
    } else if num >= 50 {
        println!("Over the correct range");
    }

    println!("~~/ Ternary operations /~~");

    let ans = if num > 40 { true } else { false };
    println!("Ternary ans: {}", ans);

    println!("~~/ loop /~~");

    'flag: loop {
        println!("Inside the first loop");
        loop {
            println!("Inside the second loop");
            loop {
                println!("Inside the third loop and exiting with break");
                break 'flag;
            }
        }
    }

    println!("~~/ for loop /~~");
    println!("iter on [2, 5, 6, 8]");
    for n in [2, 5, 6, 8].iter() {
        println!("{}", n);
    }

    println!("tuples in array");

    println!("array of (1,3), (2,4), (3,5)");
    let arr = [(1,3), (2,4), (3,5)];
    for (x, y) in arr.iter() {
        println!("(x, y) => ({}, {})", x, y);
    }

    for a in arr.iter() {
        println!("(a.0, a.1) => ({}, {})", a.0, a.1);
    }

    println!("num in 0..6");
    for num in 0..6 {
        println!("{}", num);
    }

    println!("num in 0..=6");
    for num in 0..=6 {
        println!("{}", num);
    }
}
