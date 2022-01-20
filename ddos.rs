use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::env;
use std::io;
use std::io::Write;

fn main() {
    let mut time:u64 = 0;
    let sleep = Duration::from_millis(env::args().nth(2).unwrap_or("0".to_string()).parse::<u64>().unwrap());
    loop {
        time+=1;
        thread::spawn(move|| {
            TcpStream::connect(env::args().nth(1).unwrap()).unwrap();
            print!("\r{}", time);
            io::stdout().flush().ok().expect("Could not flush stdout");
        });
        thread::sleep(sleep);
    }
}
