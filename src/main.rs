use std::time::{Instant, Duration};


fn pixelLoop(FPS: usize, update: fn(), render: fn()) {

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
            update();
            accum -= update_dt;
        }

        render();
        accum += dt;
    }
}



fn main() {
    pixelLoop(120, || {
        println!("update");
    }, || {
        
        std::thread::sleep(Duration::from_millis(16));
        println!("render");
    }
);
}
