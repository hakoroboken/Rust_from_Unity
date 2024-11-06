use fls::node::Node;
use std_msgs;

fn main(){
    let node = Node::new("Subscriber".to_string());

    let mut subscriber = node.create_subscriber::<std_msgs::msg::StringMsg>("controller".to_string());

    loop {
        let recv_msg = subscriber.subscribe();

        let get_str = format!("Receive : {}", recv_msg.data);

        node.log_info(get_str);
    }
}