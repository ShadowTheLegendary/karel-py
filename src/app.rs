use crate::iron_karel::direction::Direction;
use crate::iron_karel::{DRAW_QUEUE, DrawInfo, HEIGHT, KAREL, WIDTH, WORLD};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use pixels_util::color::{BLACK, TRANSPARENT, WHITE};
use pixels_util::util::{get_pixel, map_index_to_coordinate, set_pixel};
use std::ops::Deref;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

pub const PIXELS_PER_SQUARE: u32 = 8;
pub const PIXELS_WIDTH: u32 = PIXELS_PER_SQUARE * WIDTH;
pub const PIXELS_HEIGHT: u32 = PIXELS_PER_SQUARE * HEIGHT;

#[derive(Default)]
pub struct App {
    pub window: Option<&'static Window>,
    pub pixels: Option<Pixels<'static>>,
}

impl App {
    pub fn draw(&mut self) {
        let mut queue = DRAW_QUEUE.lock().unwrap();

        if let Some(info) = queue.pop_front() {
            drop(queue); // don't need the lock anymore
            let pixels = self.pixels.as_mut().unwrap();
            let frame = pixels.frame_mut();

            for i in info.redraws {
                let square = info.world.squares[i];
                let coordinate = map_index_to_coordinate(i, info.world.size.0);

                let flipped_y = (HEIGHT - 1) - coordinate.1;
                let start = (
                    coordinate.0 * PIXELS_PER_SQUARE,
                    flipped_y * PIXELS_PER_SQUARE,
                );
                let end = (start.0 + PIXELS_PER_SQUARE, start.1 + PIXELS_PER_SQUARE);

                let is_karel = coordinate == info.karel.position;
                let facing = if is_karel && info.redraw_karel {
                    Some(info.karel.facing)
                } else {
                    None
                };

                for x in start.0..end.0 {
                    for y in start.1..end.1 {
                        if square.color == TRANSPARENT {
                            let half = PIXELS_PER_SQUARE / 2;

                            let is_center = if PIXELS_PER_SQUARE.is_multiple_of(2) {
                                (x, y) == (start.0 + half, start.1 + half)
                                    || (x, y) == (start.0 + (half - 1), start.1 + half)
                                    || (x, y) == (start.0 + (half - 1), start.1 + (half - 1))
                                    || (x, y) == (start.0 + half, start.1 + (half - 1))
                            } else {
                                let center = (start.0 + half, start.1 + half);
                                center == (x, y)
                            };

                            set_pixel(
                                frame,
                                (x, y),
                                PIXELS_WIDTH,
                                PIXELS_HEIGHT,
                                match is_center {
                                    true => BLACK,
                                    false => WHITE,
                                },
                            );
                        } else {
                            set_pixel(frame, (x, y), PIXELS_WIDTH, PIXELS_HEIGHT, square.color);
                        }

                        if let Some(facing) = facing {
                            let s = PIXELS_PER_SQUARE - 2;
                            let (lx, ly) = (x - start.0, y - start.1);

                            let is_border = lx == 1 || lx == s || ly == 1 || ly == s;

                            let (missing_a, missing_b) = match facing {
                                Direction::North => ((1, 1), (s, 1)),
                                Direction::South => ((1, s), (s, s)),
                                Direction::East => ((s, 1), (s, s)),
                                Direction::West => ((1, 1), (1, s)),
                            };

                            if is_border && (lx, ly) != missing_a && (lx, ly) != missing_b {
                                match get_pixel(frame, (x, y), PIXELS_WIDTH, PIXELS_HEIGHT) {
                                    Some(WHITE) => {
                                        set_pixel(
                                            frame,
                                            (x, y),
                                            PIXELS_WIDTH,
                                            PIXELS_HEIGHT,
                                            BLACK,
                                        );
                                    }
                                    _ => {
                                        set_pixel(
                                            frame,
                                            (x, y),
                                            PIXELS_WIDTH,
                                            PIXELS_HEIGHT,
                                            WHITE,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            pixels.render().unwrap();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();
        let window: &'static Window = Box::leak(Box::new(window));

        let size = window.inner_size();

        let surface = SurfaceTexture::new(size.width, size.height, window);

        let pixels = PixelsBuilder::new(PIXELS_WIDTH, PIXELS_HEIGHT, surface)
            .present_mode(wgpu::PresentMode::Immediate)
            .build()
            .unwrap();

        DRAW_QUEUE.lock().unwrap().push_back(DrawInfo {
            world: WORLD.lock().unwrap().deref().deref().clone(),
            karel: *KAREL.lock().unwrap().deref(),
            redraws: (0..(WIDTH * HEIGHT) as usize).collect(), // redraw everything
            redraw_karel: true,
        });

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(new_size) => {
                self.pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(new_size.width, new_size.height)
                    .unwrap();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => self.draw(),
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}
