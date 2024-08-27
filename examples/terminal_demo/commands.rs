pub fn hello_world() {
    println!("Hello, world!");
}

pub fn send_msg(message: &str) {
    println!("Message sent: {}", message);
}

pub fn check_msg_send(arg1: &str, arg2: &str) {
    println!("Message check with args: '{}' '{}'", arg1, arg2);
}
