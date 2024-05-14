use std::collections::{HashMap, HashSet};

use std::env;
use std::io::{self, BufRead, BufReader, Read};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
#[derive(Debug, Clone)]
struct Group {
    pos: HashSet<(u32, u32)>,
    color: i32,
}

fn explore_board(board: &Vec<Vec<i32>>) -> Vec<Group> {
    let mut in_any_group: HashSet<(u32, u32)> = HashSet::new();

    let mut groups: Vec<Group> = Vec::new();

    for y in 0..15 {
        for x in 0..15 {
            let color = board[y as usize][x as usize];
            if color >= 0 && !in_any_group.contains(&(x, y)) {
                let mut group: Group = Group {
                    pos: vec![(x, y)].into_iter().collect(),
                    color: color,
                };
                in_any_group.insert((x, y));
                explore_group(x, y, color, &mut group, &mut in_any_group, &board);
                if group.pos.len() > 1 {
                    //eprintln!("Debug group... {:?}", group);
                    groups.push(group);
                }
            }
        }
    }

    return groups;
}

fn explore_group(
    x: u32,
    y: u32,
    color: i32,
    group: &mut Group,
    in_any_group: &mut HashSet<(u32, u32)>,
    board: &Vec<Vec<i32>>,
) {
    let mut search = |n_x: u32, n_y: u32| {
        if !in_any_group.contains(&(n_x, n_y)) {
            let n_col = *(&board[n_y as usize][n_x as usize]);
            if n_col == color {
                group.pos.insert((n_x, n_y));
                in_any_group.insert((n_x, n_y));
                explore_group(n_x, n_y, color, group, in_any_group, board);
            }
        }
    };
    //up
    if y > 0 {
        search(x, y - 1);
    }
    //down
    if y < 14 {
        search(x, y + 1);
    }
    //left
    if x > 0 {
        search(x - 1, y);
    }
    //right
    if x < 14 {
        search(x + 1, y);
    }
}

fn brain(board: &Vec<Vec<i32>>) -> Group {
    let mut groups = explore_board(&board);

    let mut init: HashMap<i32, usize> = HashMap::new();
    let colors = groups.iter().fold(&mut init, |c, g| {
        if let Some(counter) = c.get_mut(&g.color) {
            *counter += g.pos.len();
        } else {
            c.insert(g.color, g.pos.len());
        }
        c
    });
    //eprintln!("COLOR COUNTER : {:?}", colors);

    let lowest_color = {
        let mut colors = colors.iter().collect::<Vec<_>>();
        colors.sort_by_key(|(_a, b)| **b);
        colors[0].0
    };

    groups.reverse();
    let mut better = groups
        .iter()
        .filter(|g| g.color == *lowest_color)
        .map(|g| (g, g.pos.iter().map(|p| p.1).max().unwrap()))
        .collect::<Vec<_>>();

    better.sort_by_key(|g| g.1);
    better.reverse();

    turn_move(board, &better[0].0.pos);

    better[0].0.clone()
}

fn make_drops(board: &Vec<String>) -> Vec<String> {
    let empty = String::from("               ");
    let mut copied = board
        .iter()
        .map(|col| {
            let t = col.clone().replace("⚫", "");
            format!("{:⚫>15}", t)
        })
        .filter_map(|col| {
            if col != empty {
                Some(col.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for _i in 0..(15 - (copied.len())) {
        copied.push(empty.clone());
    }

    eprintln!("-----------------");
    copied.iter().for_each(|c| eprintln!("|{c}|"));
    eprintln!("-----------------");
    copied
}

fn turn_score(pos_len: usize) -> u32 {
    let n = pos_len - 2;
    (n * n) as u32
}

fn turn_move(board: &Vec<Vec<i32>>, group: &HashSet<(u32, u32)>) -> Vec<String> {
    let mut string_board = (0..15)
        .map(|x| {
            (0..15)
                .map(move |y| {
                    if group.contains(&(x, y)) {
                        return -1;
                    } else {
                        return board[y as usize][x as usize];
                    }
                })
                .map(|c| match c {
                    0 => '🟥',
                    1 => '🟩',
                    2 => '🟦',
                    3 => '🟨',
                    4 => '🟪',
                    _ => '⚫',
                })
                .collect::<Vec<_>>()
        })
        .map(|col| col.iter().collect::<String>())
        .collect::<Vec<_>>();

    eprintln!("-----------------");
    string_board.iter().for_each(|c| eprintln!("|{c}|"));
    eprintln!("-----------------");

    let score = turn_score(group.len());
    eprintln!("\nDROP NOW score {score}\n");

    make_drops(&mut string_board);
    string_board
}

fn raw_read<T: Read>(buf: T) -> Vec<Vec<i32>> {
    let mut r = BufReader::new(buf);
    eprintln!("vv __  raw_read __vv");
    (0..15 as usize)
        .map(|_i| {
            let mut inputs = String::new();
            r.read_line(&mut inputs).unwrap();
            eprint!("{inputs}");
            return inputs
                .split_whitespace()
                .map(|c| parse_input!(c, i32))
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>()
}

fn main() {
    if env::args().any(|a| a == "--debug") {
        let board = raw_read(io::stdin());
        brain(&board);
        eprintln!("--debug so break");
        return;
    }

    let mut total_score: u32 = 0;

    loop {
        let board = raw_read(io::stdin());
        let g = brain(&board);
        // //let (x, y) = better[0].0.pos.iter().next().unwrap();
        //return ((*x, *y), turn_score(better[0].0.pos.len()));

        let (x, y) = g.pos.iter().next().unwrap();
        let yy = 14 - y;
        total_score += turn_score(g.pos.len());
        println!("{x} {yy} with score {total_score}");
    }
}
