use std::io;
use std::io::{Write, BufReader}; 
use std::{thread, time, fs::File};
use notify_rust::Notification;
use rodio; //::{Decoder, OutputStream, source::Source};

fn countdown(min: i32, sec: i32 ) {
    println!("min = {}, sec = {}", min, sec);

    let mut total_seconds = min * 60 + sec;
    //let second = time::Duration::from_millis(1000);
    let second = time::Duration::from_secs(1);


    println!("Total en secondes : {}", total_seconds);
    
    print!("\x1B[?25l");
    while total_seconds > 0 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        print!("{:02}:{:02}\r", minutes, seconds);
        io::stdout().flush().unwrap();
        total_seconds -= 1;
        thread::sleep(second);
    }
    println!("00:00");
    print!("\x1B[?25h");
    
    Notification::new()
        .summary("Minuteur")
        .body("Le temps du minuteur est écoulé")
        .icon("firefox")
        //.timeout(Timeout::Milliseconds(10000)) 
        .show().unwrap();
}


fn main() {
    //let x = 5;
    
    println!("1) 03:30");
    println!("2) 06:00");
    println!("3) 15:00");
    println!("4) 35:00");
    println!("5) 00:10");

    let mut input = String::new();
    print!("Entrez le numéro du minuteur à utiliser : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let trim_input = input.trim(); // Produit un &str sans retour à la ligne
    
    match trim_input {
        "1" => countdown(3, 30),
        "2" => countdown(6, 0),
        "3" => countdown(15, 0),
        "4" => countdown(35, 0),
        "5" => countdown(0, 10),
        _ => println!("Entrée non valide!"),
       
    }

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");

    let file = BufReader::new(File::open("/home/robert/rust/projets/rtimer/assets/202029.wav").unwrap());
    let _sink = rodio::play(&stream_handle.mixer(), file).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(6));
}
