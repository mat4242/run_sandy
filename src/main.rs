use std::env;

use std::fs::File;
use std::error::Error;
//use std::time::Duration;
use rppal::i2c::I2c;

const I2C_ADDR: u16 = 0x9;

use std::io::{ self, BufRead, BufReader };

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn i2c_command  (i2c: &I2c, command_line: String) -> Result<u8, Box<(dyn std::error::Error + 'static)>>  {
    let bytes = command_line.as_bytes();
        
    for b in &bytes[0..] {
        let _result= i2c.smbus_send_byte (*b);
    }
    
    while 0 != i2c.smbus_receive_byte()? {
        //thread::sleep(Duration::from_secs(1));
    }
    Ok(0 as u8)
}


fn main() -> Result<(), Box<dyn Error>> {
    
    let args:Vec<String> = env::args().skip(1).collect();
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(I2C_ADDR)?;
    
    for filename in args {
        println!("{}", filename);
        
        let lines = read_lines(filename.to_string());
        for line in lines {
            let mut c_line= line.unwrap();
            c_line.push('\n');
            println!("process {}", c_line);
            i2c_command (&i2c, c_line)?;
        }
        
    }
    Ok(())

}
 
