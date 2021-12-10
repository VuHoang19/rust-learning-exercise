fn main() {
    let mut v0: Vec<i32> = vec![1,2,3];
    println!("vec0 is {:?}", v0);

    let mut vector: Vec<i32> = v0.clone();
    vector.push(4);
    println!("vector is {:?}", vector);
    for x in vector.iter() {
        println!("x is {}", x);
    }

    let mut reverse: Vec<i32> = vector.clone();
    reverse.reverse();
    println!("reverse is {:?}", reverse);
    for x in reverse.iter() {
        println!("x is {}", x)
    }

    let mut square: Vec<i32> = vector.iter().map(|x| x * x).collect();
    println!("square is {:?}", square);
    for x in square.iter() {
        println!("x is {}", x);
    }
}