mod loops;
use anyhow::{Context, Result};
use std::time::{Instant, Duration};


//Deprecated



struct State {
      updatesCalled: usize,
      rendersCalled: usize,
      timePassed: Duration,
      boxPosition: (isize, isize),
      boxDirection: (isize, isize),
      boxSize: (usize, usize),
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
        }
    }
}


fn main() -> Result<()> {
    let WIDTH = 640;
    let HEIGHT = 480;

    let context =
        loops::init_tao_window("pixel loop", WIDTH, HEIGHT).context("create tao window")?;
    let surface =
        loops::init_pixels(&context, WIDTH, HEIGHT).context("initialize pixel surface")?;

    // calling main loop
    let state = State::default();
    loops::run_with_tao_and_pixels(
        state,
        context,
        surface,
        |s, surface| {

            //Update the box position
            s.boxPosition.0 = s.boxPosition.0 + s.boxDirection.0;
            s.boxPosition.1 = s.boxPosition.1 + s.boxDirection.1;
            if s.boxPosition.0 + s.boxSize.0 as isize >= surface.width() as isize || s.boxPosition.0 < 0 {
                s.boxDirection.0 = s.boxDirection.0 * -1;
                s.boxPosition.0 = s.boxPosition.0 + s.boxDirection.0
            }
            if s.boxPosition.1 + s.boxSize.1 as isize >= surface.height() as isize || s.boxPosition.1 < 0 {
                s.boxDirection.1 = s.boxDirection.1 * -1;
                s.boxPosition.1 = s.boxPosition.1 + s.boxDirection.1
            }

            s.updatesCalled += 1;
            Ok(())
        //println!("update");
    }, 
        |s, surface, dt| {

            let width = surface.width();
            let height = surface.height();
            let buffer = surface.frame_mut();
            
            for y in 0..height {
                for x in 0..width {
                    let index = ((y * width + x) * 4) as usize ;
                    buffer[index + 0] = 0;
                    buffer[index + 1] = 0;
                    buffer[index + 2] = 0;
                    buffer[index + 3] = 255;
                }
            }

            //Render the box
            for y in s.boxPosition.1 as usize..s.boxPosition.1 as usize + s.boxSize.1 {
                for x in s.boxPosition.0 as usize..s.boxPosition.0 as usize + s.boxSize.0 {
                    let i = ((y * width as usize + x) * 4) as usize;
                    buffer[i + 0] = 255;
                    buffer[i + 1] = 255;
                    buffer[i + 2] = 0;
                    buffer[i + 3] = 255;
                }
            }

        //This stuff just checks update and render fps
        s.rendersCalled += 1;
        s.timePassed += dt;
        if(s.timePassed > Duration::from_secs(1)){
            println!("Update FPS: {:.2}", s.updatesCalled as f64 / 1f64);
            println!("Render FPS: {:.2}", s.rendersCalled as f64 / 1f64);
            s.updatesCalled = 0;
            s.rendersCalled = 0;
            s.timePassed = Duration::default();

        }
        surface.render();
        Ok(())    
            
        //println!("render");
    });
}
