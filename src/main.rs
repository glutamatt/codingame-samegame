use std::collections::{HashMap, HashSet};

use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
#[derive(Debug)]
struct Group {
    pos: HashSet<(u32, u32)>,
    color: i32,
}

fn explore_board(board: &Vec<Vec<i32>>) -> Vec<Group> {
    let mut in_any_group: HashSet<(u32, u32)> = HashSet::new();

    let mut groups: Vec<Group> = vec![];

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
                    eprintln!("Debug group... {:?}", group);
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

fn brain(board: &Vec<Vec<i32>>) -> (u32, u32) {
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

    eprintln!("COLOR COUNTER : {:?}", colors);

    let lowest_color = {
        let mut colors = colors.iter().collect::<Vec<_>>();
        colors.sort_by_key(|(a, b)| **b);
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

    return *better[0].0.pos.iter().collect::<Vec<_>>()[0];
}

fn main() {
    loop {
        let board = (0..15 as usize)
            .map(|_i| {
                let mut inputs = String::new();
                io::stdin().read_line(&mut inputs).unwrap();
                return inputs
                    .split_whitespace()
                    .map(|c| parse_input!(c, i32))
                    .collect::<Vec<_>>();
            })
            .collect::<Vec<_>>();

        let (x, y) = brain(&board);
        let yy = 14 - y;
        println!("{x} {yy} Hello SameGame\\n:-)"); // Selected tile "x y [message]".
    }
}
