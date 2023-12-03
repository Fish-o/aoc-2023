use std::cmp::max;

advent_of_code::solution!(2);
struct Grab {
    blue: u32,
    green: u32,
    red: u32,
}

struct Game {
    blue: u32,
    green: u32,
    red: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split(":"))
            .map(|mut p| (p.next().unwrap(), p.next().unwrap()))
            .map(|(game_id, game)| {
                let mut game_id = game_id.split(" ").last().unwrap().parse::<u32>().unwrap();
                let mut grabs = game.split(";");
                let grabs = grabs
                    .map(|g| {
                        println!("{}", g);
                        let colors = g.trim().split(",");
                        let mut blue = 0;
                        let mut green = 0;
                        let mut red = 0;
                        for color_grab in colors {
                            println!("{}", color_grab);
                            let mut color_grab_split = color_grab.trim().split(" ");
                            let amount = color_grab_split.next().unwrap().parse::<u32>().unwrap();
                            let color = color_grab_split.next().unwrap();
                            match color {
                                "blue" => blue += amount,
                                "green" => green += amount,
                                "red" => red += amount,
                                _ => panic!("Unknown color"),
                            }
                        }
                        Grab { blue, green, red }
                    })
                    .collect::<Vec<_>>();
                (game_id, grabs)
            })
            .filter(|(game_id, grabs)| {
                !grabs
                    .iter()
                    .any(|g| g.blue > 14 || g.green > 13 || g.red > 12)
            })
            .map(|(game_id, grabs)| game_id)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split(":"))
            .map(|mut p| (p.next().unwrap(), p.next().unwrap()))
            .map(|(game_id, game)| {
                let mut game_id = game_id.split(" ").last().unwrap().parse::<u32>().unwrap();
                let mut grabs = game.split(";");
                let grabs = grabs
                    .map(|g| {
                        println!("{}", g);
                        let colors = g.trim().split(",");
                        let mut blue = 0;
                        let mut green = 0;
                        let mut red = 0;
                        for color_grab in colors {
                            println!("{}", color_grab);
                            let mut color_grab_split = color_grab.trim().split(" ");
                            let amount = color_grab_split.next().unwrap().parse::<u32>().unwrap();
                            let color = color_grab_split.next().unwrap();
                            match color {
                                "blue" => blue += amount,
                                "green" => green += amount,
                                "red" => red += amount,
                                _ => panic!("Unknown color"),
                            }
                        }
                        Grab { blue, green, red }
                    })
                    .collect::<Vec<_>>();
                (game_id, grabs)
            })
            .map(|(game_id, grabs)| {
                let mut game = Game {
                    blue: 0,
                    green: 0,
                    red: 0,
                };
                for grab in grabs {
                    game.blue = max(game.blue, grab.blue);
                    game.green = max(game.green, grab.green);
                    game.red = max(game.red, grab.red);
                }
                (game_id, game)
            })
            .map(|(game_id, game)| game.blue * game.green * game.red)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
