pub fn hello_world() {
    println!("HelloWorld");
}

pub fn send_msg(message: &str) {
    println!("SendMsg: '{}', send msg done.", message);
}

pub fn check_msg_send(arg1: &str, arg2: &str) {
    println!("CheckMsgSend with args: '{}' '{}', check done.", arg1, arg2);
}
