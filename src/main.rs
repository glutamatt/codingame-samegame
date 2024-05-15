use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader, Read};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
struct Group {
    pos: HashSet<(u32, u32)>,
    color: char,
}

fn explore_board(board: &Vec<String>) -> Vec<Group> {
    let mut in_any_group: HashSet<(u32, u32)> = HashSet::new();

    let mut groups: Vec<Group> = Vec::new();
    for x in 0..15 {
        for y in 0..15 {
            let color = board[x as usize].chars().nth(y).unwrap();
            if color != '⚫' && !in_any_group.contains(&(x, y as u32)) {
                let mut group: Group = Group {
                    pos: vec![(x, y as u32)].into_iter().collect(),
                    color,
                };
                in_any_group.insert((x, y as u32));
                explore_group(x, y as u32, color, &mut group, &mut in_any_group, &board);
                if group.pos.len() > 1 {
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
    color: char,
    group: &mut Group,
    in_any_group: &mut HashSet<(u32, u32)>,
    board: &Vec<String>,
) {
    let mut search = |n_x: u32, n_y: u32| {
        if !in_any_group.contains(&(n_x, n_y)) {
            let n_col = board[n_x as usize].chars().nth(n_y as usize).unwrap();
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

fn brain(board: &Vec<String>) -> Option<Group> {
    let mut groups = explore_board(&board);

    let mut init: HashMap<char, usize> = HashMap::new();
    let colors = groups.iter().fold(&mut init, |c, g| {
        //eprintln!("debug group: {:?}", g);
        if let Some(counter) = c.get_mut(&g.color) {
            *counter += g.pos.len();
        } else {
            c.insert(g.color, g.pos.len());
        }
        c
    });

    if colors.is_empty() {
        return None;
    }
    eprintln!("COLOR COUNTER : {:?}", colors);
    let lowest_color = {
        let mut colors = colors.iter().collect::<Vec<_>>();
        colors.sort_by_key(|(_a, b)| **b);
        colors[0].0
    };

    eprintln!("lowest_color : {:?}", lowest_color);

    groups.reverse();
    let mut better = groups
        .iter()
        .filter(|g| g.color == *lowest_color)
        .map(|g| (g, g.pos.iter().map(|p| p.1).max().unwrap()))
        .collect::<Vec<_>>();

    better.sort_by_key(|g| g.1);
    better.reverse();

    better.get(0).map(|g| g.0.clone())
}

fn turn_score(pos_len: usize) -> u32 {
    let n = pos_len - 2;
    (n * n) as u32
}
fn raw_read<T: Read>(buf: T) -> Vec<String> {
    let mut r = BufReader::new(buf);
    eprintln!("vv __  raw_read __vv");
    let board = (0..15 as usize)
        .map(|_i| {
            let mut inputs = String::new();
            r.read_line(&mut inputs).unwrap();
            //eprint!("{inputs}");
            return inputs
                .split_whitespace()
                .map(|c| parse_input!(c, i32))
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    (0..15)
        .map(|x| {
            (0..15)
                .map(|y| board[y as usize][x as usize])
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
        .collect::<Vec<_>>()
}

fn print_debug(b: &Vec<String>) {
    eprintln!("-----------------");
    b.iter().for_each(|c| eprintln!("|{c}|"));
    eprintln!("-----------------");
}

fn board_depop(board: &Vec<String>, pos: &HashSet<(u32, u32)>) -> Vec<String> {
    board
        .iter()
        .enumerate()
        .map(|(x, col)| {
            col.chars()
                .enumerate()
                .map(move |(y, c)| {
                    if pos.contains(&(x as u32, y as u32)) {
                        return '⚫';
                    } else {
                        return c;
                    }
                })
                .collect::<String>()
        })
        .collect()
}

fn board_drop(board: &Vec<String>) -> Vec<String> {
    let empty = String::from("⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫");
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
    copied
}
#[derive(Debug)]
struct Eval {
    board: Vec<String>,
    moves: Vec<Move>,
    total_score: u32,
}

#[derive(Debug)]
struct Move {
    pos: HashSet<(u32, u32)>,
    eval: Option<Eval>,
    score: u32,
}
impl Eval {
    fn expand(&mut self) -> bool {
        if self.moves.is_empty() {
            return false;
        }

        let first_move = self.moves.first_mut().unwrap();

        let cont = first_move.simulate(&self.board);

        self.total_score = self
            .moves
            .iter()
            .map(|m| m.score + m.eval.as_ref().map(|e| e.total_score).unwrap_or(0))
            .max()
            .unwrap();

        cont
    }
}

impl Move {
    fn simulate(&mut self, board: &Vec<String>) -> bool {
        match self.eval.as_mut() {
            Some(a) => a.expand(),
            None => {
                let new_board = board_depop(board, &self.pos);
                let new_board = board_drop(&new_board);
                let groups = explore_board(&new_board);

                let moves = groups.iter().map(|g| Move {
                    eval: None,
                    pos: g.pos.clone(),
                    score: turn_score(g.pos.len()),
                });
                print_debug(&new_board);
                let moves: Vec<Move> = moves.collect();
                let moves_is_empty = moves.is_empty();
                let total_score = {
                    if moves_is_empty
                        && new_board
                            .iter()
                            .all(|s| s == "⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫")
                    {
                        1000
                    } else {
                        0
                    }
                };

                self.eval = Some(Eval {
                    total_score,
                    board: new_board,
                    moves,
                });

                !moves_is_empty
            }
        }
    }
}

fn main() {
    let board = raw_read(io::stdin());
    let groups = explore_board(&board);

    let mut root = Eval {
        total_score: 0,
        board: board,
        moves: groups
            .iter()
            .map(|g| Move {
                eval: None,
                pos: g.pos.clone(),
                score: turn_score(g.pos.len()),
            })
            .collect(),
    };

    while root.expand() {
        eprintln!("root.total_score ==> {}", root.total_score);
    }
}
