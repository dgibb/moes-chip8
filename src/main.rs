#[macro_use] extern crate nickel;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate byteorder;
extern crate ws;

use hyper::method::Method;
use nickel::{Nickel, StaticFilesHandler, HttpRouter};
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;
use ws::listen;
use std::thread;

mod Emulator;
mod TranslationCache;
mod Emitter;
mod Interpreter;
mod register_map;
mod executable_block;

fn main() {

    let emulator = Arc::new(RwLock::new(Emulator::Emulator::new()));
    let mut server = Nickel::new();

    thread::spawn(move ||{
        if let Err(error) = listen("192.168.1.161:8080", |out| {
            move |msg| {
                println!("Server got message '{}'. ", msg);
                out.send(msg)
            }
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });

    let emulator_rom = emulator.clone();
    server.add_route(Method::Post, "/sendRom", middleware!{|req|
        let mut emulator = emulator_rom.write().unwrap();
        req.origin.read_to_end(&mut emulator.rom).unwrap();
        emulator.init();
        let tru: bool = true;
        let response: String = serde_json::to_string(&tru).unwrap();
        format!("{:?}", response)
    });


    let emulator_block = emulator.clone();
    server.add_route(Method::Get, "/runBlock", middleware!{
        let mut emulator = emulator_block.write().unwrap();
        emulator.run_block();
        let emulator_state = emulator.get_state();
        let serialized = serde_json::to_string(&emulator_state).unwrap();
        println!("{:?}", serialized);
        format!("{:?}", serialized);
    });

    server.utilize(StaticFilesHandler::new("./Client"));
    server.listen("192.168.1.161:8081").unwrap();

}
