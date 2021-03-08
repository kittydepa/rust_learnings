use std::u32;

// Program that caluclates the area of a rectangle

// Part 3. Refactoring with Structs: Adding More Meaning

#[derive(Debug)] // Need this line to be able to print out struct!
struct Rectangle { // struct, stating what the type of our fields will be
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // Using &self here because Rust knows that this is an implementation of Rectangle
        self.width * self.height
    } // We do not take ownership here for the area parameters, just borrowing these values.

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self. height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { // Creating an instance with the w and h of our shape
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60, 
        height: 45,
    };


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    println!(
        "The area of the rectangle is {} square pixles.",
    rect1.area()
);
    

    // println!(
    //     "The area of the rectangle is {} square pixles.",
    //     area(&rect1)
    // );
}

// fn area(rectangle: &Rectangle) -> u32 { // Now defined with 1 parameter
//     rectangle.width * rectangle.height // much more clear than in pt2 with using 0 and 1 for tuple index values
// }

// Part 2. Refactoring with Tuples
// The code below is a good improvement, but it does not specify
// Which is the height and width of the rectangle, since tuples to no name their elements!
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixles.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1 // The first value * the second value given in our defined area 'dimensions', as noted in the line above. 
// }



// // Part 1. First, let's do it with single variables
// // The problem with the code below is that it does not show in any way
// // That the weidth and height are related, belong to the same shape
// // We can improve this by re-writing it with a tuple.

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixles",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

