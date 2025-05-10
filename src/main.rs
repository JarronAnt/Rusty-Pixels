use std::sync::Arc;
use std::time::{Instant, Duration};
use pixels::{Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::{Window, WindowBuilder};

//Depricated
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


#[derive(Default)]
struct State {
      updatesCalled: usize,
      rendersCalled: usize,
      timePassed: Duration,
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
        s.updatesCalled += 1;
        //println!("update");
    }, |s, dt, WIDTH, HEIGHT, pixels| {
        s.rendersCalled += 1;
        s.timePassed += dt;
        
        if(s.timePassed > Duration::from_secs(2)){
            println!("Update FPS: {:.2}", s.updatesCalled as f64 / 2f64);
            println!("Render FPS: {:.2}", s.rendersCalled as f64 / 2f64);
            s.updatesCalled = 0;
            s.rendersCalled = 0;
            s.timePassed = Duration::default();

        }
        std::thread::sleep(Duration::from_millis(16));
        //println!("render");
    });
}


fn pixelLoopTao<S: 'static>(mut state: S, (width, height):(u32,u32), FPS: usize, update: fn(&mut S,u32, u32), render: fn(&mut S, Duration, u32, u32, &mut Pixels)) {
    // THIS IS THE MAIN LOOP THAT USES AN ACCUMULATOR TO UPDATE AND RENDER
    if(FPS == 0 ) {
        panic!("Can't have 0 FPS")
    }

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

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        Pixels::new(width, height, surface_texture).unwrap()
    };


    let mut accum: Duration = Duration::new(0,0);
    let mut currentTime = Instant::now();
    let mut lastTime = Instant::now();;
    let update_dt =  Duration::from_nanos((1_000_000_000f64 / FPS as f64).round() as u64);


    event_loop.run(move |event, _, control_flow| {
        match event {
            // Update internal state and request a redraw
            Event::MainEventsCleared => {
                lastTime = currentTime;
                currentTime = Instant::now();
                let mut dt = currentTime - lastTime;

                // Escape hatch if update calls take to long in order to not spiral into
                // death
                if dt > Duration::from_millis(100) {
                    dt = Duration::from_millis(100);
                }

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