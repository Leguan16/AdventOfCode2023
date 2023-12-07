use std::collections::HashMap;
pub use crate::advent_day::Levels;

use crate::DayTwo;

impl Levels for DayTwo {

    fn level1() {
        let mut red_green_blue: HashMap<&str, u32> = HashMap::new();
        red_green_blue.insert("red", 12);
        red_green_blue.insert("green", 13);
        red_green_blue.insert("blue", 14);

        let mut i: u32 = 1;

        let mut pass = Vec::new();

        'game: for game in include_str!("input").lines() {
            for pull in game.split_once(":").unwrap().1.split(";") {
                let mut pulls: HashMap<&str, u32> = HashMap::new();
                for dice in pull.split(",") {
                    let one_color = dice.rsplit_once(" ").unwrap();
                    let number: u32 = one_color.0.trim().parse().unwrap();

                    pulls.insert(one_color.1, number);
                }

                for color in red_green_blue.iter() {
                    let amount = pulls.get(color.0).unwrap_or(&0);
                    if amount > &color.1 {
                        i += 1;
                        continue 'game;
                    }
                }
            }
            pass.push(i);
            i += 1;
        }

        let mut sum = 0;
        for x in pass {
            sum += x;
        }

        println!("{}", sum);
    }

    fn level2() {
        let mut red_green_blue: HashMap<&str, u32> = HashMap::new();
        red_green_blue.insert("red", 12);
        red_green_blue.insert("green", 13);
        red_green_blue.insert("blue", 14);

        let mut sum = 0;

        for game in include_str!("input").lines() {
            let mut min: HashMap<&str, u32> = HashMap::new();
            for pull in game.split_once(":").unwrap().1.split(";") {
                for dice in pull.split(",") {
                    let one_color = dice.rsplit_once(" ").unwrap();
                    let number: u32 = one_color.0.trim().parse().unwrap();

                    let color = one_color.1;
                    let amount = min.get(color).unwrap_or(&0);

                    if  amount < &number {
                        min.insert(color, number);
                    }
                }
            }

            sum += min.get("red").unwrap_or(&1) * min.get("green").unwrap_or(&1) * min.get("blue").unwrap_or(&1)
        }

        println!("{}", sum);
    }
}