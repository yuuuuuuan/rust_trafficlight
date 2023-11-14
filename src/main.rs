use std::thread;
use std::time::Duration;
#[derive(Debug)]
#[derive(Clone)]
enum State {
    None,
    Green,
    Yellow,
    Red,
}

pub trait GetValue {
    fn get_value(&self) -> u8;
}

#[derive(Clone)]
struct Light{
    state:State,
    time:u8,
    pre:State,
    next:State,
}

impl GetValue for State {
    fn get_value(&self) -> u8 {
        match self {
            State::None => 3,
            State::Green => 15,
            State::Yellow => 5,
            State::Red => 40,
        }
    }
}

fn main() {
    let mut light0 = State::None;
    let mut light1 = State::Green;
    let mut light2 = State::Yellow;
    let mut light3 = State::Red;
    println!("Now is {:?} light,it will last {} second.",light0,light0.get_value());
    println!("Now is {:?} light,it will last {} second.",light1,light1.get_value());
    println!("Now is {:?} light,it will last {} second.",light2,light2.get_value());
    println!("Now is {:?} light,it will last {} second.",light3,light3.get_value());
    let mut light = Light {
        state:State::None,
        time:State::None.get_value(),
        pre:State::None,
        next:State::Green,
    };
    loop {

        light.time = light.time - 1;
        println!("Now is {:?} light,it will last {} second.==>{}",light.state,light.state.get_value(),light.time);
        thread::sleep(Duration::from_secs(1));
        if light.time == 0 {
            light.pre = light.state.clone();
            light.state = light.next.clone();
            match light.next {
                State::Green => {light.next=State::Yellow},
                State::Yellow => {light.next=State::Red},
                State::Red => {light.next=State::Green},
                State::None => {},
            }
            light.time = light.state.get_value();
        }

    }
}