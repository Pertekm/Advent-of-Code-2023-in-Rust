use std::fs;
use regex::Regex;

fn main() {
    let file_path = "src/aocDay2_P1_Input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sum: i32 = 0;

    for text_line in contents.lines() {
      //println!("Zeile: {}", text_line);
      let game = map_text_line_to_structure(&text_line);

      println!("current sum: {} to be added by gameNr {} are red {}, green {} and blue {}", sum, game.nr, game.red_max, game.green_max, game.blue_max);
      sum += game.red_max * game.green_max * game.blue_max as i32;
    }
    println!("Sum: {}", sum);
}

struct Game {
    nr: i8,
    blue_max: i32,
    red_max: i32,
    green_max: i32,
}

fn map_text_line_to_structure(text_line: &str) -> Game {
  let re_game_nr = Regex::new(r"Game (?<nr>\d*):").unwrap();
  let Some(found_game_nr) = re_game_nr.captures(text_line) else { panic!() };
  //println!("Game Nr: {}", &found_game_nr["nr"]);

  let re_blue_cube_count = Regex::new(r"(?<blue>\d*) blue").unwrap();
  let found_blue_cube_count: Vec<i32> = re_blue_cube_count.find_iter(text_line).map(|m| m.as_str().replace("blue", "").trim().parse().unwrap()).collect();
  //println!("Found blue cubes: {:?}", found_blue_cube_count);
  //for blue in &found_blue_cube_count {
  //  println!("blue: {}", blue);
  //}
  //println!("max blue: {:?}", found_blue_cube_count.iter().max().unwrap());

  let re_red_cube_count = Regex::new(r"(?<red>\d*) red").unwrap();
  let found_red_cube_count: Vec<i32> = re_red_cube_count.find_iter(text_line).map(|m| m.as_str().replace("red", "").trim().parse().unwrap()).collect();
  //println!("max red: {:?}", found_red_cube_count.iter().max().unwrap());

  let re_green_cube_count = Regex::new(r"(?<green>\d*) green").unwrap();
  let found_green_cube_count: Vec<i32> = re_green_cube_count.find_iter(text_line).map(|m| m.as_str().replace("green", "").trim().parse().unwrap()).collect();
  //println!("max green: {:?}", found_green_cube_count.iter().max().unwrap());

  let game = Game {
      nr: found_game_nr["nr"].parse().unwrap(),
      blue_max: *found_blue_cube_count.iter().max().unwrap(),
      red_max: *found_red_cube_count.iter().max().unwrap(),
      green_max: *found_green_cube_count.iter().max().unwrap()
  };

  return game;
}
