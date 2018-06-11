#![feature(custom_derive, plugin)]
#![feature(rustc_private)]

extern crate bytebuffer;
extern crate byteorder;
extern crate market;
extern crate redis;
extern crate redis_client;
extern crate serde;
extern crate serde_json;
extern crate serialize;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;


#[macro_use]
extern crate serde_derive;

extern crate toml;

// use data_structure::tree;
mod JsonData;
mod RedisTest;
mod config;
mod model;
mod packet;
mod process_byte;
mod ringqueue;
mod run;
mod stream_data;
mod test_some;
mod tree;

use log::Level;

fn set_u16_le(a: &mut [u8], v: u16) -> u16 {
    print!("{:?}", a);
    a[0] = v as u8;
    a[1] = (v >> 8) as u8;
    print!("{}", v);
    print!("{:?}", a);
    return v;
}


pub fn init_log(){
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .init();
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}

fn main() {
    // run::run();
    init_log();
    run::run_thread();
    // data_structure:: tree::run();
    // tree::run();
    // ringqueue::run();
    // test_some::run();
    // test_some::run_clone_data();

    // process_byte::test_cast();
    // process_byte::test_pack_unpack();

    // let mut data = {"length":5,"id":1,"msg_type":1,"msg":"abc"};
    // let mut data = "{\"id\":64,\"title\":\"24days\",\"stats\":{\"pageviews\":1500}}".to_string();
    // JsonData::str_data(data);
    // RedisTest::set_and_get();
    // RedisTest::do_something();

    //   stream_data::head();
    // config::load();

    // let mut end:u16 = 0;
    //   let mut buff = [2,5];
    //   print!("{:?}",buff);
    //   end = set_u16_le( &mut buff ,end);
    //   println!("{}",end);
    //   let mut price = packet::Head::new([128, 0, 1, 33].to_vec());
    //     let clone = price.clone();
    //     let mut wt_buffer = price.pack();
    //     let head = packet::Head::unpack(wt_buffer);
    //         println!("{}",head.MsgSize);
    // println!("{}",head.MsgType);
    // assert_eq!(clone.MsgSize, unpack.MsgSize)
}
