//20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20

use std::net::UdpSocket;
use fls::node::Node;
use std_msgs;

const RECV_ADDR: &str = "127.0.0.1:64276";

fn main() {
    let mut n = 0;

    let mut node = Node::new("Publisher".to_string());

    let mut pu = node.create_publisher::<std_msgs::msg::StringMsg>("controller".to_string());

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
                                let indices = [2, 3, 0, 4, 5, 17, 15];

                                let mut new_msg = std_msgs::msg::StringMsg::new();
                                let mut msg_data = String::new();

                                for &index in indices.iter() {
                                    if !msg_data.is_empty() {
                                        msg_data.push(',');
                                    }
                                    msg_data.push_str(&numbers[index as usize].to_string());
                                }
                                
                                msg_data.push('e');
                                println!("{}",msg_data);
                                new_msg.data = msg_data;
                                pu.publish(new_msg);

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