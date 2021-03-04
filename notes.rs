// Print lyrics to The Twelve Days of Xmas

    // let opening = "On the { number } day of Christmas my true love gave to me "
    // let first = "first item"
    // let second = "a patr"

    // put in an index too e.g index = []
    // let mut song_line = 0;
    // see https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops

    fn main() {
        let first = "On the 1st day of Xmas...";
        let second = "On the 2nd day of Xmas...";
        let third = "On the 3rd day of Xmas...";

        let mut song_line = 1;

        let song = loop {
            song_line += 1;

            if song_line == 13 {
                break
            }
        };
    
        //for song in 
    }
    
    
    
    // Pt 1. Fahrenheit to Celcius Converter
    // fn main() {
    //    let answer = temp_convert(50.0);
    
    //    println!("The answer is: {}", answer);
    // }
    
    // fn temp_convert(x: f64) -> f64 {
    //     (x - 32.0) * (5.0 / 9.0)
    // }
    