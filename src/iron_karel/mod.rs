pub mod direction;
pub mod karel;
pub mod world;

use crate::iron_karel::direction::Direction;
use crate::iron_karel::karel::Karel;
use crate::iron_karel::world::{World, WorldSquare};
use pixels_util::color::color_from_string;
use pixels_util::util::map_coordinate_to_index;
use pyo3::exceptions::PyRuntimeError;
use pyo3::{PyResult, pyfunction};
use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::{LazyLock, Mutex};

pub const WIDTH: u32 = 32;
pub const HEIGHT: u32 = 32;

/// The global Karel object used
pub static KAREL: Mutex<Karel> = Mutex::new(Karel::DEFAULT);

/// The global world object used
pub static WORLD: Mutex<LazyLock<World>> = Mutex::new(LazyLock::new(|| World {
    size: (WIDTH, HEIGHT),
    squares: vec![WorldSquare::DEFAULT; (WIDTH * HEIGHT) as usize],
}));

/// The queue that new world states are held in until [crate::run_karel_program] is called
pub static DRAW_QUEUE: Mutex<VecDeque<DrawInfo>> = Mutex::new(VecDeque::new());

/// a single world state, containing all the information needed to draw it
pub struct DrawInfo {
    pub world: World,
    pub karel: Karel,
    /// The specific world squares to redraw
    pub redraws: Vec<usize>,
    pub redraw_karel: bool,
}

/// Rotates [KAREL] 90° counter-clockwise
#[pyfunction]
pub fn turn_left() {
    KAREL.lock().unwrap().facing.shl();
    update(Vec::new(), true);
}

/// Rotates [KAREL] 90° clockwise
#[pyfunction]
pub fn turn_right() {
    KAREL.lock().unwrap().facing.shr();
    update(Vec::new(), true);
}

/// Rotates [KAREL] 180°
#[pyfunction]
pub fn turn_around() {
    KAREL.lock().unwrap().facing.flp();
    update(Vec::new(), true);
}

/// Moves [KAREL] one space in the direction it is currently facing
/// Calls [crash] if the new position would be out of bounds
#[pyfunction]
pub fn r#move() -> PyResult<()> {
    let new_position = {
        let karel = KAREL.lock().unwrap();
        karel.facing.mve(karel.position)
    };

    if let Some(new_position) = new_position {
        let mut karel = KAREL.lock().unwrap();
        let old_position = karel.position;
        karel.position = new_position;
        drop(karel);
        update(vec![map_coordinate_to_index(old_position, WIDTH)], true);
    } else {
        return crash("Tried to move out of bounds!".to_string());
    }

    Ok(())
}

/// Paints the square at the position of [KAREL] the color passed. Supports most colors you can think of.
/// Will not add a new state to the [DRAW_QUEUE] if the color of the square does not change
#[pyfunction]
pub fn paint(color: String) -> PyResult<()> {
    if let Some(color) = color_from_string(color.clone()) {
        let karel = KAREL.lock().unwrap();
        let mut world = WORLD.lock().unwrap();
        let mut square = world.get(karel.position);
        if square.color == color {
            return Ok(());
        }
        square.color = color;
        world.set(karel.position, square);
    } else {
        return crash(format!(r#"No such color: "{}"!"#, color));
    }
    let karel_pos = KAREL.lock().unwrap().position;
    update(vec![map_coordinate_to_index(karel_pos, WIDTH)], true);

    Ok(())
}

/// Returns if [KAREL] is currently facing north
#[pyfunction]
pub fn facing_north() -> bool {
    KAREL.lock().unwrap().facing == Direction::North
}

/// Returns if [KAREL] is currently facing east
#[pyfunction]
pub fn facing_east() -> bool {
    KAREL.lock().unwrap().facing == Direction::East
}

/// Returns if [KAREL] is currently facing south
#[pyfunction]
pub fn facing_south() -> bool {
    KAREL.lock().unwrap().facing == Direction::South
}

/// Returns if [KAREL] is currently facing west
#[pyfunction]
pub fn facing_west() -> bool {
    KAREL.lock().unwrap().facing == Direction::West
}

/// Returns if the square in front of [KAREL] is in bounds.
/// If this function returns true, calling move immediately after is guaranteed to succeed
#[pyfunction]
pub fn front_is_clear() -> bool {
    let karel = KAREL.lock().unwrap();
    let position = karel.position;
    karel.facing.mve(position).is_some()
}

/// Adds the current state of the world to the [DRAW_QUEUE]
pub fn update(redraws: Vec<usize>, redraw_karel: bool) {
    let karel = KAREL.lock().unwrap();
    let world = WORLD.lock().unwrap();

    let mut queue = DRAW_QUEUE.lock().unwrap();
    queue.push_back(DrawInfo {
        world: world.deref().deref().clone(),
        karel: *karel.deref(),
        redraws,
        redraw_karel,
    });
}

/// Ends the program with an error message
pub fn crash(msg: String) -> PyResult<()> {
    let karel = KAREL.lock().unwrap();

    let msg = format!(
        "Karel crashed at ({}, {}) facing {:?}:\n              {}",
        karel.position.0, karel.position.1, karel.facing, msg
    );

    Err(PyRuntimeError::new_err(msg))
}
