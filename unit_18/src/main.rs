fn main() {
    // Patterns and Matching 

    ///patterns are a flexible way to implement control flow depending 
    /// on what value something is
    
    let x: Option<i32> = Some(10);

    match x {
        Some(num) => println!("x is {}", num),
        None => println!("x is null"),
    };

    //if let is another way to do "if its Some, print it"

    if let Some(num) = x { //if x is the Some() variant, take the value in it
        println!("the value was {}", num);
    }//note: we couldnt do another condition on num bc it's
    //only a valid name once the curly braces start

    //the _ is a catchall pattern in the end of a match that matches with
    //anything but doesnt bind to anything

    //we can use while let to run a loop while a pattern matches

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {//becomes None if there is nothing to pop
        println!("{} was on top", top);
    };

    //tuple matching

    let v = vec![1,2,3,4,5];

    for (index, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    };

    //every let statement matches a pattern

    let (a,b,c) = (1,2,3); //is valid

    //functions also do pattern matching

    fn print_coordinates(&(x, y): &(i32, i32)) { //note use of & on left side
        println!("current location is {}, {}", x, y);
    }

    let point: (i32, i32) = (3, 4);
    print_coordinates(&point);

    ///one concept in patterns is irrefutability and refutability
    /// an irrefutable pattern is one that matches anything, like
    /// let x = 5;
    /// an a refutable one is one that may not match something
    /// like let Some(num) = BLAH. if BLAH is None, it won't match
    

    //you can use | for or and .. for ranges in match statements

    let x = 5;

    match x {
        1 | 2 => println!("1 or 2"),
        3..=5 => println!("3 to 5"),
        _ => println!("catchall!"),
    };

    //you can automatically unpack struct fields if you want

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point {x: 0, y: 7};

    match p {
        Point { x, y: 0 } => println!("on the x axis"),
        Point { x: 0, y } => println!("on the y axis"),
        Point { x, y } => println!("neither!"),
    }

    //you can destructure nested structs, enums, and tuples

    //_ can be matched with anything, and you can use it multiple times
    //it doesnt bind to anything. it is a special reserved character

    //prefacing a variable name with _ suppresses compiler wardnings about it
    //being unused

    //you can use .. to absorb multiple parts of an object
    let (x, ..) = (1, 3, 4);

    //you can add extra conditions to match statements with Match Guards

    let x = Some(9);

    match x {
        Some(num) if num % 2 == 0 => println!("even number"),
        _ => println!("even number or None"),
    };

    //the @ symbol can be used to create a variable and also test it for
    //a condition within a match statement

    





    

    







    



}
