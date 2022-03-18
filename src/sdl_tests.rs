// Requires all lib files from: SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\
// to be put in: C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-msvc\lib
// AND SDL2.dll from: SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\
// to be put into the project directory NEXT to the Cargo.toml file
//
// Or, linux:
// sudo apt install libsdl2-dev
//
// A very good tutorial for SDL based audio output is here: https://stasiak.at/updating-sdl2-audio-callback-data-in-rust.html
// It should be saved in Internet Archive's Wayback Machine

extern crate sdl2; // https://lib.rs/crates/sdl2

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::Duration;

use sdl2::audio::{AudioCallback, AudioSpecDesired};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
    pwm: f32
}

impl SquareWave {
    fn update_pwm(&mut self, new_pwm: f32) {
        println!("Updating PWM to {:?}", new_pwm);
        self.pwm = new_pwm;
        if self.pwm > 0.9999 {
            self.pwm = 0.9999
        }
        if self.pwm < 0.0001 {
            self.pwm = 0.0001
        }
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    // callback is called when the audio subsystem needs more audio sample data.
    fn callback(&mut self, out: &mut [f32]) {
        // Iterate across each of the samples in the `out` array and set their volume to correctly generate a square wave: 
        for x in out.iter_mut() {
            
            // Each sample should be 100% or 0% based on the current phase and the PWM amount:
            *x = if self.phase <= self.pwm {
                self.volume
            } else {
                -self.volume
            };

            // Every sample, update the phase based on the current phase and the phase increment amount:
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };
    
    // let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
    //     // initialize the audio callback
    //     SquareWave {
    //         phase_inc: 440.0 / spec.freq as f32,
    //         phase: 0.0,
    //         volume: 0.25,
    //         pwm: 0.9
    //     }
    // }).unwrap();

    let mut my_audio_callback = SquareWave {
        phase_inc: 440.0 / 44100.0,
        phase: 0.0,
        volume: 0.25,
        pwm: 0.9
    };

    println!("Initial pwm value: {:?}", my_audio_callback.pwm);
    my_audio_callback.update_pwm(0.5);
    println!("New, initial pwm value: {:?}", my_audio_callback.pwm);
    

    // Open an audio device for playback at the sample rate defined in desired_spec, and set up the AudioCallback:
    let mut audio_device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
        my_audio_callback
    }).unwrap();
 
    // Create a new SDL window:
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    // Create a new SDL canvas in the window:
    let mut canvas = window.into_canvas().build().unwrap();
 
    // Draw some stuff to the canvas:
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // Start audio playback. The AudioCallback will start being called when the audio subsystem needs more sample data:
    audio_device.resume();
    
    // Play for 2 seconds
    // std::thread::sleep(Duration::from_millis(2000));

    // Keep track of any previously pressed mouse buttons:
    let mut prev_buttons = HashSet::new();

    // Start grabing events from the SDL Context (the event_pump will "pump" events out of the context so we can handle them):
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    // This is the main SDL processing loop:
    'running: loop {

        // Draw some pretty color cycles to the canvas:
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // Poll the event pump for important events (mostly keyboard events here):
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // Drop out of the game loop if the escape key is hit
                    break 'running
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here...

        // do mouse stuff
        let state = event_pump.mouse_state();
        let buttons = state.pressed_mouse_buttons().collect();
        // get difference in button state:
        let new_buttons = &buttons - &prev_buttons;
        let old_buttons = &prev_buttons - &buttons;

        // Handle mouse button state change:
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            // One of the buttons changed!!

            // println!(
            //     "X = {:?}, Y = {:?} : {:?} -> {:?}",
            //     state.x(),
            //     state.y(),
            //     new_buttons,
            //     old_buttons
            // )
        }
        prev_buttons = buttons;

        // Mouse x/y location logging:
        // println!(
        //     "X = {:?}, Y = {:?}",
        //     state.x() as f32/800.0,
        //     state.y()
        // );

        ::std::thread::sleep(Duration::from_millis(10));

        // Lock the audio device so the audio callback gets dereferenced so we can access it and modify it:
        let mut lock = audio_device.lock();

        // Attempt to update PWM value of AudioCallback in real time:
        let new_pwm_value = state.x() as f32/800.0;
        lock.update_pwm(new_pwm_value);

        // Render the canvas:
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // this feels a little... mathy
        
        // Sleep until the next frame:
        ::std::thread::sleep(Duration::from_millis(10));
    }
}
