//20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20
//Tsuna

use std::net::UdpSocket;
use serialport;

const RECV_ADDR: &str = "127.0.0.1:64276";

fn main() {
    let mut n = 0;

    let mut port = serialport::new("/dev/ttyUSB0", 115200)
        .timeout(std::time::Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    match UdpSocket::bind(RECV_ADDR) {
        Ok(sock) => {
            loop {
                let mut buff = [0; 53];
                match sock.recv_from(&mut buff) {
                    Ok((recv_size, _src)) => {
                        match String::from_utf8(buff[..recv_size].to_vec()) {
                            Ok(v) => {
                                n+=1;
                                println!("-------------------------------------------------------");
                            	println!("DataNumber:\n{}",n);
                                println!("AllDate:\n{}", v);
                                let numbers: Vec<i32> = v.split(',') 
                                    .filter_map(|s| s.parse::<i32>().ok()) 
                                    .collect();

                                println!("DateDetail:");
                                let indices = [3, 1, 4];
                                
                                let mut msg_data = String::new();

                                for &index in indices.iter() {
                                    if !msg_data.is_empty() {
                                        msg_data.push(',');
                                    }
                                    msg_data.push_str(&numbers[index as usize].to_string());
                                }
                                msg_data.push_str(",20e");
                                
                                match port.write(msg_data.as_bytes()) {
                                    Ok(_)=>{
                                        println!("{}", msg_data);
                                    }
                                    Err(e)=> println!("Failed to write to serial port: {}", e),
                                }
                            }
                            Err(e) => println!("Failed to convert to string from u8 array: {}", e),
                        }
                    },
                    Err(e) => println!("Failed to receive message: {}", e),
                }
            }
        },
        Err(e) => println!("Failed to start UDP receiver: {}", e),
    }
}