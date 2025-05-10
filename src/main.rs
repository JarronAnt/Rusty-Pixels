use std::sync::Arc;
use std::time::{Instant, Duration};
use pixels::{Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::{Window, WindowBuilder};

//Deprecated
fn pixelLoop<S>(mut state: S, FPS: usize, update: fn(&mut S), render: fn(&mut S, Duration)) {
    // THIS IS THE MAIN LOOP THAT USES AN ACCUMULATOR TO UPDATE AND RENDER
    if(FPS == 0 ) {
        panic!("Can't have 0 FPS") 
    }


    let mut accum: Duration = Duration::new(0,0);
    let mut currentTime = Instant::now();
    let mut lastTime;
    let update_dt =  Duration::from_nanos((1_000_000_000f64 / FPS as f64).round() as u64);


    loop{

        lastTime = currentTime;
        currentTime = Instant::now();
        let mut dt = currentTime - lastTime;

        // escape trigger so not to have a death loop
        if dt > Duration::from_millis(100) {
            dt = Duration::from_millis(100);
        }

        while(accum > update_dt) {
            update(&mut state);
            accum -= update_dt;
        }

        render(&mut state, dt );
        accum += dt;
    }
}


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


fn main() {
    let WIDTH = 640;
    let HEIGHT = 480;

    // calling main loop
    let state = State::default();
    pixelLoopTao(
        state,
        (WIDTH as u32,HEIGHT as u32),
        120,
        |s, WIDTH, HEIGHT| {

            //Update the box position
            s.boxPosition.0 = s.boxPosition.0 + s.boxDirection.0;
            s.boxPosition.1 = s.boxPosition.1 + s.boxDirection.1;
            if s.boxPosition.0 + s.boxSize.0 as isize >= WIDTH as isize || s.boxPosition.0 < 0 {
                s.boxDirection.0 = s.boxDirection.0 * -1;
                s.boxPosition.0 = s.boxPosition.0 + s.boxDirection.0
            }
            if s.boxPosition.1 + s.boxSize.1 as isize >= HEIGHT as isize || s.boxPosition.1 < 0 {
                s.boxDirection.1 = s.boxDirection.1 * -1;
                s.boxPosition.1 = s.boxPosition.1 + s.boxDirection.1
            }

            s.updatesCalled += 1;
        //println!("update");
    }, |s, dt, WIDTH, HEIGHT, pixels| {

            //setup of the buffer to draw to the screen & clear background
            let buffer = pixels.frame_mut();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let index = ((y * WIDTH + x) * 4) as usize ;
                    buffer[index + 0] = 0;
                    buffer[index + 1] = 0;
                    buffer[index + 2] = 0;
                    buffer[index + 3] = 255;
                }
            }

            //Render the box
            for y in s.boxPosition.1 as usize..s.boxPosition.1 as usize + s.boxSize.1 {
                for x in s.boxPosition.0 as usize..s.boxPosition.0 as usize + s.boxSize.0 {
                    let i = ((y * WIDTH as usize + x) * 4) as usize;
                    buffer[i + 0] = 255;
                    buffer[i + 1] = 255;
                    buffer[i + 2] = 0;
                    buffer[i + 3] = 255;
                }
            }

        //slow rendering down to 60ish fps
        std::thread::sleep(Duration::from_millis(9));

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
        //println!("render");
    });
}

// THIS IS THE MAIN LOOP THAT USES AN ACCUMULATOR TO UPDATE AND RENDER
fn pixelLoopTao<S: 'static>(mut state: S, (width, height):(u32,u32), FPS: usize, update: fn(&mut S,u32, u32), render: fn(&mut S, Duration, u32, u32, &mut Pixels)) {
    if(FPS == 0 ) {
        panic!("Can't have 0 FPS")
    }

    //create window and event loop
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title("Hello Pixels/Tao")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    //create window
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        Pixels::new(width, height, surface_texture).unwrap()
    };

    //timer stuff setup
    let mut accum: Duration = Duration::new(0,0);
    let mut currentTime = Instant::now();
    let mut lastTime = Instant::now();
    let update_dt =  Duration::from_nanos((1_000_000_000f64 / FPS as f64).round() as u64);


    //main event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            // Update internal state
            Event::MainEventsCleared => {
                //Timer stuff
                lastTime = currentTime;
                currentTime = Instant::now();
                let mut dt = currentTime - lastTime;

                // Escape hatch if update calls take to long in order to not spiral into
                // death
                if dt > Duration::from_millis(100) {
                    dt = Duration::from_millis(100);
                }

                //update at fixed rate
                while accum > update_dt {
                    update(&mut state, width, height);
                    accum -= update_dt;
                }
                render(&mut state, dt, width, height, &mut pixels);
                accum += dt;

                if let Err(err) = pixels.render() {
                    panic!("Pixels render error");
                }
            }

            _ => {}
        }
    });

}