use std::collections::LinkedList;
pub use crate::advent_day::{Levels};

use crate::DayOne;

impl Levels for DayOne {
    fn level1() {
        let split = include_str!("input").lines();
        let mut sum = 0;

        for line in split {
            sum += calc(line);
        }
        println!("The number is: {}", sum);
    }

    fn level2() {
        let split = include_str!("input").lines();

        let mut sum = 0;

        for line in split {
            let mut abc = line.replace("one", "o1e");
            abc = abc.replace("two", "t2o");
            abc = abc.replace("three", "t3e");
            abc = abc.replace("four", "f4r");
            abc = abc.replace("five", "f5e");
            abc = abc.replace("six", "s6x");
            abc = abc.replace("seven", "s7n");
            abc = abc.replace("eight", "e8ht");
            abc = abc.replace("nine", "ni9ne");

            sum += calc(&*abc);
        }

        println!("{}", sum);
    }
}

fn calc(line: &str) -> u32 {
    let mut list = LinkedList::new();

    for char in line.chars() {
        if char.is_digit(10) {
            list.push_back(char.to_digit(10).unwrap());
        }
    }

    let front = list.pop_front().unwrap();
    let back: u32;

    if list.len() > 0 {
        back = list.pop_back().unwrap();
    } else {
        back = front;
    }

    let num = front * 10 + back;
    return num;
}