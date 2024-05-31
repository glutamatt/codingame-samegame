use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader, Read};
use std::rc::Rc;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
struct Group {
    pos: HashSet<(u32, u32)>,
    min_y: u32,
    color: char,
}

fn explore_board(board: &[String]) -> Vec<Group> {
    let mut in_any_group: HashSet<(u32, u32)> = HashSet::new();

    let mut groups: Vec<Group> = Vec::new();
    for x in 0..15 {
        for y in 0..15 {
            let color = board[x as usize].chars().nth(14 - y as usize).unwrap();
            if color != '⚫' && !in_any_group.contains(&(x, y)) {
                let mut group: Group = Group {
                    pos: vec![(x, y)].into_iter().collect(),
                    color,
                    min_y: 0,
                };
                in_any_group.insert((x, y));
                explore_group(x, y, color, &mut group, &mut in_any_group, &board);
                if group.pos.len() > 1 {
                    group.min_y = group.pos.iter().map(|(_, y)| *y).min().unwrap();
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
    board: &[String],
) {
    let mut search = |n_x: u32, n_y: u32| {
        if !in_any_group.contains(&(n_x, n_y)) {
            let n_col = board[n_x as usize].chars().nth(14 - n_y as usize).unwrap();
            if n_col == color {
                group.pos.insert((n_x, n_y));
                in_any_group.insert((n_x, n_y));
                explore_group(n_x, n_y, color, group, in_any_group, board);
            }
        }
    };
    if y > 0 {
        search(x, y - 1);
    }
    if y < 14 {
        search(x, y + 1);
    }
    if x > 0 {
        search(x - 1, y);
    }
    if x < 14 {
        search(x + 1, y);
    }
}

fn turn_score(pos_len: usize) -> u32 {
    let n = pos_len - 2;
    (n * n) as u32
}
fn raw_read<T: Read>(buf: T) -> Vec<String> {
    let mut r = BufReader::new(buf);
    let int_board = (0..15 as usize)
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
                .map(|y| int_board[y as usize][x as usize])
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

fn print_debug(b: &[String]) {
    eprintln!("-----------------");
    for y in 0..15 {
        let row = (0..15)
            .map(|x| b[x].chars().nth(y).unwrap())
            .collect::<String>();
        eprintln!("|{row}|");
    }

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
                    if pos.contains(&(x as u32, 14 - y as u32)) {
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
    explored: bool,
}

#[derive(Debug)]
struct Move {
    pos: HashSet<(u32, u32)>,
    eval: Option<Rc<RefCell<Eval>>>,
    score: u32,
    color: char,
    min_y: u32,
}

impl Eval {
    fn show_path(&self) {
        print_debug(&self.board);
        let next = self.moves.iter().max_by_key(|&m| {
            m.eval
                .as_ref()
                .map(|r| {
                    let rr = r.as_ref();
                    let total = rr.borrow().total_score;
                    total + m.score
                })
                .unwrap_or(0)
        });

        if let Some(best_move) = next {
            if let Some(eval) = best_move.eval.as_ref() {
                eval.borrow().show_path();
            }
        }
    }

    fn rank_groups(&mut self) {
        let mut init: HashMap<char, usize> = HashMap::new();

        let colors = self.moves.iter().fold(&mut init, |c, g| {
            if let Some(counter) = c.get_mut(&g.color) {
                *counter += g.pos.len();
            } else {
                c.insert(g.color, g.pos.len());
            }
            c
        });
        if colors.is_empty() {
            return;
        }
        let lowest_color = {
            let (c, _) = colors.iter().min_by_key(|(_, s)| *s).unwrap();
            c
        };
        self.moves.sort_unstable_by(|a, b| {
            if a.color == *lowest_color && b.color != *lowest_color {
                return Ordering::Less;
            }
            if b.color == *lowest_color && a.color != *lowest_color {
                return Ordering::Greater;
            }
            a.min_y.cmp(&b.min_y)
        });
    }

    fn expand(&mut self, memo: &mut HashMap<String, Rc<RefCell<Eval>>>) -> bool {
        if !self.moves.is_empty() {
            let mut uncharted_moves = self
                .moves
                .iter_mut()
                .filter(|m| match &m.eval {
                    Some(e) => !e.as_ref().borrow().explored,
                    _ => true,
                })
                //.choose(&mut rand::thread_rng());
                //.next();
                .collect::<Vec<_>>();

            let chosen = uncharted_moves.choose_weighted_mut(&mut rand::thread_rng(), |m| m.score);

            if chosen.unwrap().simulate(&self.board, memo) {
                self.explored = self.moves.iter().all(|m| {
                    m.eval
                        .as_ref()
                        .map(|e| e.as_ref().borrow().explored)
                        .unwrap_or(false)
                });
            }
        }

        self.total_score = self
            .moves
            .iter()
            .map(|m| {
                m.score
                    + m.eval
                        .as_ref()
                        .map(|e| e.as_ref().borrow().total_score)
                        .unwrap_or(0)
            })
            .max()
            .unwrap();

        !self.explored
    }
}

impl Move {
    fn simulate(
        &mut self,
        board: &Vec<String>,
        memo: &mut HashMap<String, Rc<RefCell<Eval>>>,
    ) -> bool {
        match self.eval.as_mut() {
            Some(a) => a.borrow_mut().expand(memo),
            None => {
                let new_board = board_depop(board, &self.pos);
                let new_board = board_drop(&new_board);

                let cache_key = new_board.join("");

                if let Some(cached) = memo.get(&cache_key) {
                    //println!("cache hit");
                    let cloned = cached.clone();
                    let explored = cloned.as_ref().borrow().explored;
                    self.eval = Some(cloned);
                    !explored
                } else {
                    //println!("cache miss");
                    let groups = explore_board(&new_board);

                    let moves = groups.iter().map(|g| Move {
                        color: g.color,
                        min_y: g.min_y,
                        eval: None,
                        pos: g.pos.clone(),
                        score: turn_score(g.pos.len()),
                    });
                    let moves: Vec<Move> = moves.collect();
                    let explored = moves.is_empty();
                    let total_score = {
                        if explored
                            && new_board
                                .iter()
                                .all(|s| s == "⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫")
                        {
                            1000
                        } else {
                            0
                        }
                    };

                    let mut new_eval = Eval {
                        total_score,
                        board: new_board,
                        moves,
                        explored,
                    };
                    new_eval.rank_groups();
                    let new_eval = Rc::new(RefCell::new(new_eval));
                    memo.insert(cache_key, new_eval.clone());
                    self.eval = Some(new_eval);
                    !explored
                }
            }
        }
    }
}

fn main() {
    let board = raw_read(io::stdin());
    let groups = explore_board(&board);

    let mut root = Eval {
        total_score: 0,
        explored: false,
        board: board,
        moves: groups
            .iter()
            .map(|g| Move {
                eval: None,
                min_y: g.min_y,
                color: g.color,
                pos: g.pos.clone(),
                score: turn_score(g.pos.len()),
            })
            .collect(),
    };

    root.rank_groups();

    let mut max_score = 0;

    let mut memo: HashMap<String, Rc<RefCell<Eval>>> = HashMap::new();

    while root.expand(&mut memo) {
        if root.total_score > max_score {
            max_score = root.total_score;

            root.show_path();

            //let mut current = Some(&root);
            //let mut sc = 0;
            //while current.is_some() {
            //    print_debug(&current.unwrap().board);
            //    current = current
            //        .unwrap()
            //        .moves
            //        .iter()
            //        .filter(|m| m.eval.is_some())
            //        .max_by_key(|m| m.eval.as_ref().unwrap().total_score + m.score)
            //        .map(|m| {
            //            sc += m.score;
            //            eprintln!("m.score {} -> {}", m.score, sc);
            //            m.eval.as_ref().unwrap()
            //        })
            //}

            eprintln!("root.total_score ==> {}", root.total_score);
        }
    }
    eprintln!("root.total_score ==> {}", root.total_score);
}
