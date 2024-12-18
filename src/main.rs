use ev3dev_lang_rust::{sensors::InfraredSensor, sound, Ev3Result, Led};
use std::{thread, time};

fn main() -> Ev3Result<()> {
    let ir = InfraredSensor::find().expect("IR sensor not connected");
    let led = Led::new().expect("LED error");

    ir.set_mode_ir_prox().expect("Failed to set IR to proximity mode");
    led.set_color(Led::COLOR_OFF)?; // Stops the startup animation

    let colors = vec![
        Led::COLOR_YELLOW, 
        Led::COLOR_AMBER,  
        Led::COLOR_RED
    ];

    for i in 0..colors.len() {
        println!("Arming in {}...", colors.len() - i);

        led.set_color(colors[i])?;
        sleep(1.0);
    }

    led.set_color(Led::COLOR_OFF)?;
    
    let threshold = 2;
    let init_dist = ir.get_distance().expect("Failed to get distance");

    println!("Alarm armed");
    println!("Initial distance: {}", init_dist);

    loop {
        match ir.get_distance() {
            Ok(dist) => {
                if (dist - init_dist).abs() >= threshold {
                    println!("MOTION DETECTED!");
                    
                    let mut switch = false;
                    
                    loop {
                        switch = !switch;
                        
                        let freq = if switch {
                            led.set_color(Led::COLOR_RED)?;

                            500.0
                        } else {
                            led.set_color(Led::COLOR_OFF)?;

                            1000.0
                        };

                        sound::tone(freq, 50)?.wait()?;
                    }
                }
            },

            Err(err) => {
                println!("Failed to get distance: {:?}", err);
                continue;
            }
        }
    }
}

fn sleep(s: f32) {
    thread::sleep(time::Duration::from_secs_f32(s));
}