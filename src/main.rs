use std::time::{Instant, Duration};


fn pixelLoop<S>(mut state: S, FPS: usize, update: fn(&mut S), render: fn(&mut S, Duration)) {

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
    let state = State::default();


    pixelLoop(state,120, |s| {
        s.updatesCalled += 1;
        //println!("update");
    }, |s, dt| {
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
