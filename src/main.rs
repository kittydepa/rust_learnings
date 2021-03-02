

// fn main() {
//     let x = 5.0;
    
//     let x =  x + 1.0;

//     let x = x * 2.0;

//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is {}", y); // This is called deconstructing
// }

// you can also access the tuple value by using a period like so:
fn main(){
    let x : (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
} // This creates a tuple called 'x'
