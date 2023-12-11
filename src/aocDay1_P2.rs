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

      // ToDo: bei 'nineight' -> 9ight statt nin8 als Ergebnis erwartet. Also zuerst die erste Ziffer (vom String-Anfang) extrahieren.
      let mut text_line_converted_to_digit = text_line.replace("one", "1");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("two", "2");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("three", "3");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("four", "4");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("five", "5");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("six", "6");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("seven", "7");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("eight", "8");
      text_line_converted_to_digit = text_line_converted_to_digit.replace("nine", "9");

      //println!("ori: {}, conv: {}", text_line, text_line_converted_to_digit);
            
      for text_char in text_line_converted_to_digit.chars() {
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
