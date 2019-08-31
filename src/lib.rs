extern crate js_sys;
mod color;
mod rotations;
use std::ops::Add;
use std::ops::Sub;
use wasm_bindgen::prelude::*;
// use web_sys::console;

use std::cell::RefCell;

thread_local!(static PIECES_GLOBAL: RefCell<Vec<Vec<Piece>>> = RefCell::new(Vec::new()));
thread_local!(static HOLES_GLOBAL: RefCell<Vec<u128>> = RefCell::new(Vec::new()));
thread_local!(static SOLUTION_GLOBAL: RefCell<Vec<(usize, usize, usize)>> = RefCell::new(Vec::new()));

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
    fill_color: i32,
    edge_color: i32,
}

impl From<(i32, i32, i32)> for Block {
    fn from((x, y, z): (i32, i32, i32)) -> Block {
        Block {
            x: x,
            y: y,
            z: z,
            ..Default::default()
        }
    }
}

impl From<i32> for Block {
    fn from(position: i32) -> Block {
        Block::from((position % 5, (position % 25) / 5, position / 25))
    }
}

impl Into<i32> for &Block {
    fn into(self) -> i32 {
        i32::from(self.x) + i32::from(self.y) * 5 + i32::from(self.z) * 5 * 5
    }
}

impl Sub for &Block {
    type Output = Block;
    fn sub(self, other: &Block) -> Block {
        Block {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            fill_color: self.fill_color,
            edge_color: self.edge_color,
        }
    }
}
impl Add for &Block {
    type Output = Block;
    fn add(self, other: &Block) -> Block {
        Block {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            fill_color: self.fill_color,
            edge_color: self.edge_color,
        }
    }
}
impl Into<u128> for Block {
    fn into(self) -> u128 {
        let position: i32 = (&self).into();
        if !(0..125).contains(&position) {
            panic!("Bits outside cube.")
        };
        1 << position
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Block {}

#[derive(Serialize, Deserialize, Clone)]
struct Piece {
    blocks: [Block; 5],
    fill_color: i32,
    edge_color: i32,
    position: i32,
    // #[serde(skip)]
    rotations: Vec<Piece>,
    bits: u128,
    number: usize,
}

impl Default for Piece {
    fn default() -> Piece {
        Piece {
            blocks: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            fill_color: 0,
            edge_color: 0,
            position: 0,
            rotations: Vec::new(),
            bits: 0,
            number: 0,
        }
    }
}

impl Into<u128> for &Piece {
    fn into(self) -> u128 {
        self.blocks
            .iter()
            .map(|block| {
                let position: i32 = block.into();
                if !(0..125).contains(&position) {
                    panic!("Bits outside cube.")
                };
                1 << position
            })
            .fold(0, |acc, x| x | acc)
    }
}

impl Into<u128> for &mut Piece {
    fn into(self) -> u128 {
        self.blocks
            .iter()
            .map(|block| {
                let position: i32 = block.into();
                1 << position
            })
            .fold(0, |acc, x| x | acc)
    }
}
#[derive(Serialize, Deserialize)]
struct SoutionResult {
    solution: Vec<(usize, usize, usize)>,
    solved: bool,
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// fn convertBlocks(blocks: &mut Vec<Block>) -> [Block; 5] {
//     [blocks.remove(0), blocks[1], blocks[2], blocks[3], blocks[4]]
// }

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    // console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn draw_solution_js(js_objects: JsValue);
}
fn draw_solution(piece_set: Vec<Vec<Piece>>, solution: &Vec<(usize, usize, usize)>) {
    let sol: Vec<Piece> = solution
        .iter()
        .map(|(i, p, r)| piece_set[*i][*p].rotations[*r].clone())
        .collect();
    draw_solution_js(JsValue::from_serde(&sol).unwrap());
}

use std::cell::RefMut;

#[wasm_bindgen]
pub fn solve_step_js(steps: i32, check_flats: bool, check_holes: bool) -> JsValue {
    let mut solution_found: bool = solve_step(steps, check_flats, check_holes);
    let solution: Vec<(usize, usize, usize)> =
        SOLUTION_GLOBAL.with(|solution_global_cell| solution_global_cell.borrow().clone());

    let result = SoutionResult {
        solution: solution,
        solved: solution_found,
    };
    JsValue::from_serde(&result).unwrap()
}
fn solve_step(steps: i32, check_flats: bool, check_holes: bool) -> bool {
    let piece_set: Vec<Vec<Piece>> =
        PIECES_GLOBAL.with(|pieces_global_cell| pieces_global_cell.borrow().clone());

    SOLUTION_GLOBAL.with(|solution_global_cell| {
        let mut sol_ref: RefMut<Vec<(usize, usize, usize)>> = solution_global_cell.borrow_mut();
        let solution: &mut Vec<(usize, usize, usize)> = sol_ref.as_mut();

        let mut solution_found: bool = false;

        for step in 0..steps {
            let mut cube: u128 = 0;
            let mut used: [bool; 25] = [false; 25];
            for (i, p, r) in solution.iter() {
                let bits: u128 = piece_set[*i][*p].rotations[*r].bits;
                cube = cube | bits;
                used[*p] = true;
            }
            let mut i = match solution.last() {
                Some(x) => x.0,
                None => 0,
            };
            //seek to next empty spot
            while cube & (1 << i) > 0 {
                i += 1;
            }
            // console_log!("i:{}", i);
            let mut p: usize = match (0usize..25).find(|x| !used[*x]) {
                Some(x) => x,
                None => 25,
            };

            //sanity checks

            //flat blocks and last layer
            let mut step_back: bool = false;
            if check_flats && i > 99 && used[0..8].iter().find(|u| !**u).is_some() {
                step_back = true;
            }

            // let mut step_back = false;
            // holes
            if check_holes {
                HOLES_GLOBAL.with(|solution_global_cell| {
                    let holes = solution_global_cell.borrow().clone();
                    for index in i..125 {
                        if cube & (1 << index) == 0 {
                            // console_log!("{} {} {}", holes.len(), index, holes[index]);
                            if cube & holes[index] == holes[index] {
                                step_back = true;
                                // console_log!(
                                //     "Hole {} {} {}  {} ",
                                //     index,
                                //     cube,
                                //     holes[index],
                                //     cube & holes[index]
                                // );
                            }
                        }
                    }
                });
            }

            // console_log!("p:{}", p);
            if p != 25 {
                let mut r = 0;
                //find a piece that fits
                loop {
                    // console_log!("r:{}", r);
                    if step_back || piece_set[i][p].rotations.len() == r {
                        r = 0;
                        loop {
                            p += 1;
                            if p == 25 || !used[p] {
                                break;
                            }
                        }
                        if step_back || p == 25 {
                            let (i2, p2, r2) = solution.pop().unwrap();
                            i = i2;
                            p = p2;
                            r = r2;
                            used[p] = false;
                            cube = cube & !piece_set[i][p].rotations[r].bits;
                            r = r + 1;
                            step_back = false;
                            // console_log!("stepping back; :{} {} {}", i, p, r);
                            continue;
                        }
                        // console_log!("p:{}", p);
                        continue;
                    }

                    let bits: u128 = piece_set[i][p].rotations[r].bits;
                    if cube & bits == 0 {
                        break;
                    }
                    r += 1;
                }
                if p == 25 {
                    //failed, go back
                } else {
                    used[p] = true;
                    cube = cube | piece_set[i][p].rotations[r].bits;
                    solution.push((i, p, r));
                }
            } else {
                //Solution found
                solution_found = true;
                break;
            }
        }
        solution_found
    })
}

fn rotate_block(translated_block: Block, rot: &Vec<i32>) -> Block {
    Block {
        x: rot[0] * translated_block.x + rot[3] * translated_block.y + rot[6] * translated_block.z,
        y: rot[1] * translated_block.x + rot[4] * translated_block.y + rot[7] * translated_block.z,
        z: rot[2] * translated_block.x + rot[5] * translated_block.y + rot[8] * translated_block.z,
        fill_color: translated_block.fill_color,
        edge_color: translated_block.edge_color,
        ..Default::default()
    }
}

use std::convert::TryFrom;

//get all 24 possible rotations
#[wasm_bindgen]
pub fn get_rotations(js_objects: &JsValue) -> JsValue {
    let piece: Piece = js_objects.into_serde().unwrap();
    let rots = rotations::get_rots();
    let mut rotations: Vec<Piece> = Vec::new();
    for rot in rots.iter() {
        let fixed_block = &piece.blocks[0];
        let mut blocks: Vec<Block> = piece
            .blocks
            .into_iter()
            .map(|block| {
                let translated_block = block - fixed_block;
                rotate_block(translated_block, rot)
            })
            .collect();
        for x in 0..5 {
            let index = i32::try_from(x).unwrap();
            blocks[x].fill_color = color::Color::from((50, index * 20 + 100, 50)).into();
        }
        let rotation = Piece {
            blocks: [
                blocks.remove(0),
                blocks.remove(0),
                blocks.remove(0),
                blocks.remove(0),
                blocks.remove(0),
            ],
            number: piece.number,
            fill_color: piece.fill_color,
            ..Default::default()
        };
        rotations.push(rotation);
    }
    JsValue::from_serde(&rotations).unwrap()
}

fn is_isomorphic(blocks_a: &[Block], blocks_b: &[Block]) -> bool {
    for a in blocks_a {
        let found = blocks_b
            .into_iter()
            .find(|b| b.x == a.x && b.y == a.y && b.z == a.z);
        if found.is_none() {
            return false;
        }
    }
    true
}

#[wasm_bindgen]
pub fn get_legal_rotations_js(piece_js: &JsValue, position: i32) -> JsValue {
    let piece: Piece = piece_js.into_serde().unwrap();
    let rots = rotations::get_rots();
    // console_log!("All rots {}", rots.len());
    let rots = get_basic_rots(&piece, &rots);
    // console_log!("Legal rots basic {}", rots.len());
    let rotations = get_legal_rotations(&piece, position, &rots);
    JsValue::from_serde(&rotations).unwrap()
}

#[wasm_bindgen]
pub fn get_legal_rotations_all_js(js_objects: &JsValue) -> JsValue {
    let pieces: Vec<Piece> = js_objects.into_serde().unwrap();
    let rots = rotations::get_rots();
    let mut piece_set = get_legal_rotations_all(pieces, &rots);

    // thread_local!(static piecesGlobal: RefCell<Vec<Piece>> = RefCell::new(Vec::new()));

    for index in piece_set.iter_mut() {
        for piece in index.iter_mut() {
            for rotation in piece.rotations.iter_mut() {
                rotation.bits = rotation.into();
            }
        }
    }

    for index in piece_set.iter() {
        let p = index.clone();
        PIECES_GLOBAL.with(|pieces_global_cell| {
            pieces_global_cell.borrow_mut().push(p);
        });
    }

    HOLES_GLOBAL.with(|holes_global_cell| {
        *holes_global_cell.borrow_mut() = get_holes();
    });

    JsValue::from_serde(&piece_set).unwrap()
}

fn get_legal_rotations_all(pieces: Vec<Piece>, rots: &Vec<Vec<i32>>) -> Vec<Vec<Piece>> {
    let mut all_pieces: Vec<Vec<Piece>> = Vec::new();
    let mut basic_rots = Vec::new();
    for piece in pieces.iter() {
        basic_rots.push(get_basic_rots(piece, rots));
    }
    for position in 0..125 {
        // console_log!("{}", position);
        let mut pieces_at_position: Vec<Piece> = Vec::new();
        for piece in pieces.iter() {
            let mut piece: Piece = piece.clone();
            piece.rotations = get_legal_rotations(&piece, position, &basic_rots[piece.number]);
            pieces_at_position.push(piece);
        }
        all_pieces.push(pieces_at_position);
    }

    all_pieces
}

fn get_basic_rots(piece: &Piece, rots: &Vec<Vec<i32>>) -> Vec<Piece> {
    let mut rotations: Vec<Piece> = Vec::new();
    let position: i32 = 62;
    let p: Block = position.into();
    for rot in rots.iter() {
        for fixed_block in piece.blocks.iter() {
            let mut blocks: Vec<Block> = piece
                .blocks
                .iter()
                .map(|block| {
                    let mut cloned_block = block.clone();
                    if cloned_block == *fixed_block {
                        cloned_block.fill_color =
                            color::Color::from(piece.fill_color).darken(50).into();
                    }
                    let translated_block = &cloned_block - fixed_block;
                    let rotated_block = &rotate_block(translated_block, rot);
                    rotated_block + &p
                })
                .filter(|block| {
                    let block_position: i32 = block.into();
                    block_position >= position
                        && (0..5).contains(&block.x)
                        && (0..5).contains(&block.y)
                        && (0..5).contains(&block.z)
                })
                .collect();
            // console_log!("Rot: {:?}", blocks);
            if blocks.len() == 5 {
                let found = rotations.iter().find(|r| is_isomorphic(&r.blocks, &blocks));
                if found.is_none() {
                    for x in 0..5 {
                        if blocks[x].fill_color == 0 {
                            blocks[x].fill_color = color::Color::from(piece.fill_color)
                                .lighten(i32::try_from(x).unwrap() * 5)
                                .into();
                        }
                    }

                    let rotation = Piece {
                        blocks: [
                            blocks.remove(0),
                            blocks.remove(0),
                            blocks.remove(0),
                            blocks.remove(0),
                            blocks.remove(0),
                        ],
                        number: piece.number,
                        ..Default::default()
                    };
                    rotations.push(rotation);
                }
            }
        }
    }
    rotations
}

fn get_legal_rotations(piece: &Piece, position: i32, rots: &Vec<Piece>) -> Vec<Piece> {
    let mut rotations: Vec<Piece> = Vec::new();
    let p: Block = position.into();
    let offset: Block = 62.into();
    for rot in rots.iter() {
        let mut blocks: Vec<Block> = rot
            .blocks
            .iter()
            .map(|block| &(block + &p) - &offset)
            .filter(|block| {
                (0..5).contains(&block.x) && (0..5).contains(&block.y) && (0..5).contains(&block.z)
            })
            .collect();
        // console_log!("Rot: {:?}", blocks);
        if blocks.len() == 5 {
            let rotation = Piece {
                blocks: [
                    blocks.remove(0),
                    blocks.remove(0),
                    blocks.remove(0),
                    blocks.remove(0),
                    blocks.remove(0),
                ],
                number: piece.number,
                fill_color: piece.fill_color,
                ..Default::default()
            };
            rotations.push(rotation);
        }
    }
    rotations
}

fn get_holes() -> Vec<u128> {
    let mut holes: Vec<u128> = Vec::new();
    for i in 0..125 {
        let mut hole: u128 = 0;
        let block: Block = Block::from(i);
        if (0..5).contains(&(block.x - 1)) {
            let bits: u128 = (&block + &Block::from((-1, 0, 0))).into();
            hole = hole | bits;
        }
        if (0..5).contains(&(block.x + 1)) {
            let bits: u128 = (&block + &Block::from((1, 0, 0))).into();
            hole = hole | bits;
        }
        if (0..5).contains(&(block.y - 1)) {
            let bits: u128 = (&block + &Block::from((0, -1, 0))).into();
            hole = hole | bits;
        }
        if (0..5).contains(&(block.y + 1)) {
            let bits: u128 = (&block + &Block::from((0, 1, 0))).into();
            hole = hole | bits;
        }
        if (0..5).contains(&(block.z - 1)) {
            let bits: u128 = (&block + &Block::from((0, 0, -1))).into();
            hole = hole | bits;
        }
        if (0..5).contains(&(block.z + 1)) {
            let bits: u128 = (&block + &Block::from((0, 0, 1))).into();
            hole = hole | bits;
        }
        holes.push(hole);
    }
    holes
}
