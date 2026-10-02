pub mod app;
pub mod iron_karel;

use crate::app::App;
use iron_karel::*;
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, pyfunction, pymodule, wrap_pyfunction};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use winit::platform::wayland::EventLoopBuilderExtWayland;

#[pyfunction]
pub fn run_karel_program() -> PyResult<()> {
    println!("Running Karel Program!");
    let event_loop =
        EventLoopBuilderExtWayland::with_any_thread(&mut EventLoopBuilder::default(), true)
            .build()
            .unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();

    Ok(())
}

#[pymodule]
fn karel_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(turn_left, m)?)?;
    m.add_function(wrap_pyfunction!(turn_right, m)?)?;
    m.add_function(wrap_pyfunction!(turn_around, m)?)?;
    m.add_function(wrap_pyfunction!(r#move, m)?)?;
    m.add_function(wrap_pyfunction!(paint, m)?)?;
    m.add_function(wrap_pyfunction!(facing_north, m)?)?;
    m.add_function(wrap_pyfunction!(facing_east, m)?)?;
    m.add_function(wrap_pyfunction!(facing_south, m)?)?;
    m.add_function(wrap_pyfunction!(facing_west, m)?)?;
    m.add_function(wrap_pyfunction!(front_is_clear, m)?)?;
    m.add_function(wrap_pyfunction!(run_karel_program, m)?)?;
    Ok(())
}
