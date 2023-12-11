use std::fs;

fn main() {
    let file_path = "src/aocDay1_P1_Input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sum = 0;

    for text_line in contents.lines() {
      //println!("Zeile: {}", text_line);
      let mut first_digit: Option<u32> = None;
      let mut last_digit: Option<u32> = None;
      
      for text_char in text_line.chars() {
        let number = text_char.to_digit(10);
        //println!("Char: {} Index: {}, number: {:?}", text_char, index_char, number);
        if number.is_some() {
          if first_digit.is_some() {
              last_digit = number;
          } else {
             first_digit = number; 
          }
        }
      }

      if first_digit.is_some() && !last_digit.is_some() {
        last_digit = first_digit;
      }
      
      if first_digit.is_some() && last_digit.is_some() {
        let new_digit = first_digit.unwrap() * 10 + last_digit.unwrap();
        println!("First Digit: {:?}, last digit: {:?}, new digit: {}",first_digit, last_digit, new_digit);
        sum += new_digit;
      }
    }
  println!("Sum: {}", sum);
}
