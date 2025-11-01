use std::io;
use std::env;
use std::io::{Write, BufReader}; 
use std::{thread, time, fs::File};
use notify_rust::Notification;
use rodio; //::{Decoder, OutputStream, source::Source};

fn countdown(min: i32, sec: i32 ) {
    let mut total_seconds = min * 60 + sec;
    //let second = time::Duration::from_millis(1000);
    let second = time::Duration::from_secs(1);

    
    print!("\x1B[?25l"); // Hide cursor
    while total_seconds > 0 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        print!("{:02}:{:02}\r", minutes, seconds);
        io::stdout().flush().unwrap();
        total_seconds -= 1;
        thread::sleep(second);
    }
    println!("00:00");
    print!("\x1B[?25h"); // Show cursor
    
    Notification::new()
        .summary("Rtimer")
        .body("Time is up!")
        //.timeout(Timeout::Milliseconds(10000)) 
        .show().unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let time = &args[1].clone();
    let min = &time[0..2];
    let sec = &time[3..5];
    
    let min_int: i32 = min.parse().unwrap();
    let sec_int: i32 = sec.parse().unwrap();

    countdown(min_int, sec_int); 

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");

    let file = BufReader::new(File::open("/home/robert/rust/projets/rtimer/assets/202029.wav").unwrap());
    let _sink = rodio::play(&stream_handle.mixer(), file).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(6));
}
