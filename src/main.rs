use std::process::Command;
use std::io::{BufWriter, Write};
use std::fs::File;
//use rand::Rng;

fn main() {
    let mut l = 8.;
    while l < 45.0 {
        let mut s = format!("./evolve.in");
        let mut buffer = File::create(s).unwrap(); 
        let mass = l;
        let z = 0.02;
        writeln!(buffer, "{} {} {}",mass, z, 12000);
        writeln!(buffer, "{} {} {} {}",0.5,0.0,0.5,190.0);
        writeln!(buffer, "{} {} {} {} {} {}",0,1,0,1,3.0,999);
        writeln!(buffer, "{} {} {}",0.05,0.01,0.02);

        let output = Command::new("./sse")
        .output()
        .expect("failed to start `ls`");
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
        l += 0.1;
    }
}
