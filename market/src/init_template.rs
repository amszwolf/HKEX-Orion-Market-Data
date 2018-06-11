use model::*;
use RedisTest;

use redis_client;

use redis;
use redis::{Connection,Commands,FromRedisValue};
use std::collections::HashMap;

pub struct HKData{
    pub redis_conn: redis::Connection
}

impl HKData{

    pub fn new(redis_addr:&str) ->Self{
        HKData{
            redis_conn:RedisTest::redis_conn(redis_addr)
        }
        
    }

    pub fn hk_quote(con: &redis::Connection, key:&str) ->  Price{
        // let client = try!(redis::Client::open("redis://192.168.1.11"));
        // let con = try!(client.get_connection());
        
        // let hk_price = redis::cmd(cmd).arg(args).query(&self.redis_conn).unwrap();
        let map : HashMap<String, String> = con.hgetall("hk_quote").unwrap();
        // for (key,value) in map{
        //     if key == String::from("00700"){
        //         let price :Price = Price::to_obj(value).unwrap();
        //         return price;
        //     }
            
        // }
        let value = map.get("00700");
        println!("key {} value {:?}",key,value );
        // let value = map.get("00700");
        let price :Price = Price::to_obj(value.unwrap().as_str()).unwrap();
        return price;

        // if !map.contains_key(key) {
        //     let value = map.get("00700");
        //      let price :Price = Price::to_obj(value.unwrap().as_str()).unwrap();
        //     return price;
        // }else{
        //     Price::other()
        // }
    }

    fn do_something(con: &redis::Connection) -> redis::RedisResult<()> {
    let _ : () = try!(redis::cmd("SET").arg("my_key").arg(42).query(con));
    Ok(())
    }
}



pub fn test_init_template(){
    let conn = RedisTest::redis_conn("redis://192.168.1.11");
    // let hk_data = HKData::new("redis://192.168.1.11");
    let price  = HKData::hk_quote(&conn,"00700");
    println!("stock_code {}",price.symbol );
}
