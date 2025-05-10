mod loops;
use anyhow::{Context, Result};
use std::time::{Duration};
use tao::event::{ElementState, Event, MouseButton, WindowEvent};



//Deprecated



struct State {
      updatesCalled: usize,
      rendersCalled: usize,
      timePassed: Duration,
      boxPosition: (isize, isize),
      boxDirection: (isize, isize),
      boxSize: (usize, usize),
      buttonPressed: bool, 
      cursorPosition: (usize, usize),
}

impl Default for State {
    fn default() -> Self {
        Self {
            updatesCalled: Default::default(),
            rendersCalled: Default::default(),
            timePassed: Default::default(),
            boxPosition: Default::default(),
            boxDirection: (2, 2),
            boxSize: (50, 50),
            buttonPressed: false,
            cursorPosition: (0,0),
        }
    }
}


fn main() -> Result<()> {
    let WIDTH = 640;
    let HEIGHT = 480;
    let scale = 1;

    let context =
        loops::init_tao_window("pixel loop", WIDTH, HEIGHT).context("create tao window")?;
    let surface =
        loops::init_pixels(&context, WIDTH / scale, HEIGHT / scale).context("initialize pixel surface")?;

    // calling main loop
    let state = State::default();
    loops::run_with_tao_and_pixels(
        state,
        context,
        surface,
        |s, surface| {
            s.updatesCalled += 1;
            //START UPDATE
            //END UPDATE
            Ok(())
        //println!("update");
    },
        |s, surface, dt| {
            let width = surface.width();
            let height = surface.height();
            let buffer = surface.frame_mut();

            //START RENDER 
            for y in 0..height {
                for x in 0..width {
                    let index = ((y * width + x) * 4) as usize;
                    buffer[index + 0] = 0;
                    buffer[index + 1] = 0;
                    buffer[index + 2] = 0;
                    buffer[index + 3] = 0;
                }
            }
            //END RENDER 

            //This stuff just checks update and render fps
            s.rendersCalled += 1;
            s.timePassed += dt;
            if (s.timePassed > Duration::from_secs(1)) {
                println!("Update FPS: {:.2}", s.updatesCalled as f64 / 1f64);
                println!("Render FPS: {:.2}", s.rendersCalled as f64 / 1f64);
                s.updatesCalled = 0;
                s.rendersCalled = 0;
                s.timePassed = Duration::default();
            }
            surface.render()?;
            Ok(())
        },
        //println!("render");
            |s, surface, _, event| {
                match event {
                    Event::WindowEvent {
                        event: win_event, ..
                    } => match win_event {
                        WindowEvent::MouseInput {
                            button: MouseButton::Left,
                            state,
                            ..
                        } => {
                            if state == &ElementState::Pressed {
                                s.buttonPressed = true;
                            } else {
                                s.buttonPressed = false;
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let position = (position.x as f32, position.y as f32);
                            let pixel_position = surface
                                .pixels()
                                .window_pos_to_pixel(position)
                                .unwrap_or((0, 0));
                            s.cursorPosition = pixel_position;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                Ok(())
    });
}
