use config::*;
use std::str::FromStr;
use std::thread;
use stream_data;



pub fn run_thread() {
    let conf: Conf = load_ip_config();

    for x in conf.ip_config.unwrap() {
        // let port = x.port.unwrap();
        // let port_name = port;
        let child = thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let addr = format!("{}:{}", x.ip.unwrap(),  x.port.unwrap());
            println!("{}", addr);
               stream_data::head(addr);
            // the lock is unlocked here when `data` goes out of scope.
        });
        let res = child.join();
    }
}
