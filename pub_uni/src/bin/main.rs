//20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20

use std::net::UdpSocket;
use fls::node::Node;
use std_msgs;

const RECV_ADDR: &str = "127.0.0.1:64276";

fn main() {
    let mut n = 0;

    let mut node = Node::new("Publisher".to_string());

    let mut pu = node.create_publisher::<std_msgs::msg::Int32>("controller".to_string());

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
                                let indices = [2, 3, 0, 4, 5];
                                for &index in indices.iter() {
                                    if index < numbers.len() as i32 {
                                        let mut new_msg = std_msgs::msg::Int32::new();
                                        println!("{}", numbers[index as usize]);
                                        new_msg.data = numbers[index as usize];
                                        pu.publish(new_msg);
                                    }
                                }

                                // for num in numbers {
                                //     let mut new_msg = std_msgs::msg::Int32::new();
                                //     new_msg.data = num as i32;
                                //     pu.publish(new_msg);
                                // }
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