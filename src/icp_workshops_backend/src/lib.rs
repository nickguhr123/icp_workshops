use candid::types::number::Nat;
use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn send_chat(in_msg: String) {
    CHAT.with(|chat| chat.borrow_mut().push(in_msg));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT.with(|chat| (*chat.borrow()).clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
