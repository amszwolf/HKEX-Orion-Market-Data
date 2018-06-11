use std::io::prelude::*;
use std::net::TcpStream;

use bytebuffer::ByteBuffer;
// use packet::{save_data, AddOddLotOrder, Head, IndexDefinition, NominalPrice, NowPrice, PackHead, LevelPrice};
use packet::*;

// if buf_data == 0:
//     self.create()
//     buf = b''
//     continue
// buf += buf_data

// while (self.runing) :
//     if len(buf) < 16 : break
//     header = get_packet_header(buf[:16])
//     if len(buf) < header['PktSize']: break
//     if header['MsgCount'] == 0:
//         buf = buf[header['PktSize']:]; break
//     data = get_packet_data(buf[:header['PktSize']])
//     if data :
//         total += 1
//         data['SeqNum'] = header['SeqNum']
//         data['SendTime'] = header['SendTime']/1000000
//         self.handle(data)
//     buf = buf[header['PktSize']:]

pub fn head(addr: String) {
    static buff_status: usize = 4096;

    let mut stream = TcpStream::connect(addr).unwrap();
    let mut body = Vec::new();
    let mut pkt_size = 0;
    let mut msg_count = 0;
    let mut buff = [0; 4096];
    let mut pack_end_flag: bool = true;
    let mut pack;
    let mut i = 1;
    loop {
        let _ = stream.read(&mut buff); // ignore here too
                                        // println!("{:?}",buff);
        let data = buff.clone();
        // println!("{:?}",data );
        body.extend(data.to_vec());
        println!("recv data {}", i = i + 1);
        // continue;
        if i == 10 {
            break;
        }
        loop {
            println!(" body.len() {}", body.len());
            if body.len() <= 16 {
                break;
            }
            pack = PackHead::write_bytes(body.clone());

            pkt_size = pack.PktSize;
            msg_count = pack.MsgCount;
            println!("size {}", pkt_size);
            println!("msg_count {}", msg_count);
            println!("SeqNum {}", pack.SeqNum);
            println!(" time {}", pack.SendTime);
            if msg_count == 0 {
                body = body.split_off(pkt_size as usize);

                break;
            }
            if pkt_size == 0 {
                println!(
                    "recv new pack body.len {} pkt_size {} ",
                    body.len(),
                    pkt_size
                );
                break;
            }
            if body.len() < pkt_size as usize {
                println!(
                    "recv new pack body.len {} pkt_size {} ",
                    body.len(),
                    pkt_size
                );
                break;
            }

            // let mut body_clone = body.clone();
            // let mut body_pack = body.split_off(16);
            // let mut body_pack = body;
            // body_pack.body,(pkt_size as usize);
            let mut body_pack = body.clone();
            body_pack = body_pack.split_off(16);
            let end = get_packet_data(body_pack);
            if end == -1 {
                break;
            }

            //  let mut body_pack = body.clone();
            body.split_off(pkt_size as usize);
            println!(
                "recv new pack body.len {} pkt_size {} ",
                body.len(),
                pkt_size
            );
            if body.len() == 304 {
                break;
            }
            // let mut body_pack = body.clone();
            // body_pack.split_off(pkt_size as usize);
            // get_packet_data(body_pack);
            // body = body.split_off(pkt_size as usize);
            // body = body_pack_end;
        }
        break;
    }
}

// }
//         // println!("{:?}", data.to_vec());
//         pack = PackHead::write_bytes(body.clone());
//         pkt_size = pack.PktSize;
//         msg_count =  pack.MsgCount;
//         if (body.len() >= pkt_size as usize) {
//             println!("pkt_size {}",pkt_size );
//             let mut body_clone = body.clone();
//             let body_pack = body_clone.split_off(16);
//             let body_pack_clone = body_pack.clone();
//             let body_head = Head::new(body_pack_clone);
//             println!("MsgType {}", body_head.MsgType);
//             println!("MsgSize {}", body_head.MsgSize);
//             if (body_head.MsgType == 53) {
//                 println!("MsgType :{}", body_head.MsgType);
//                 println!("MsgSize :{}", body_head.MsgSize);

//                 let filler_nums = (pkt_size - 12) / 24;
//                 println!(" filler_nums {}",filler_nums );
//                 let body_data = LevelPrice::new(body_pack, filler_nums as u8);
//                 println!("{}", body_data);
//             }
//             let( _, body ) = body.split_at(pkt_size as usize);
//             i += 1;
//             println!("I: {}",i );
//             continue;
//         }
//         if (msg_count == 0){
//             println!("msg_count is zero");
//             let( _, body ) = body.split_at(pkt_size as usize);
//             continue;
//         }

//     }
// }

pub fn head_bak(addr: String) {
    static buff_status: usize = 4096;

    let mut stream = TcpStream::connect(addr).unwrap();
    // let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    // ignore the Result
    // let _ = stream.write(&[1])
    let mut body = Vec::new();
    let mut pkt_size = 0;
    let mut buff = [0; 4096];
    let mut pack_end_flag: bool = true;
    let mut pack;
    loop {
        let _ = stream.read(&mut buff); // ignore here too
                                        // println!("{:?}",buff);
        let data = buff.clone();

        // println!("{:?}", data.to_vec());

        pack = PackHead::write_bytes(buff.to_vec());
        // println!("{}", pack);
        // println!("{}", pack.PktSize);
        // println!("{}", pack.SendTime);

        if (body.is_empty() || pack_end_flag) {
            pkt_size = pack.PktSize
        } else if (!pack_end_flag || body.len() >= 16) {
            pack = PackHead::write_bytes(body.clone());
            pkt_size = pack.PktSize;
        }
        // if (body.len() < (pack.PktSize as usize)) {
        body.extend(data.to_vec());
        buff = [0; 4096];

        if (body.len() >= pkt_size as usize) {
            let body_clone = body.clone();
            let body_pack = body.split_off(16);
            let body_pack_clone = body_pack.clone();
            let body_head = Head::new(body_pack_clone);
            // println!("MsgType :{}", body_head.MsgType);
            // println!("MsgSize :{}", body_head.MsgSize);

            if (body_head.MsgType == 11) {
                let body_data = NowPrice::new(body_pack);
                // println!("{:?}", &body_pack_clone);
                // println!("{}", body_data.MsgSize);
                // println!("{}", body_data.MsgType);
                // println!("{}", body_data.SecurityCode);
                // println!("{}", body_data.MarketCode);
                // break;
                // if (body_data.MsgType == 11) {
                // let body_pack_clone = body_pack.clone();
                // let body_data = NowPrice::new(body_pack);
                // // println!("{:?}", &body_pack_clone);
                println!("{}", body_data.MsgSize);
                println!("{}", body_data.MsgType);
                println!("{}", body_data.SecurityCode);
                println!("{}", body_data.MarketCode);

            // }
            } else if (body_head.MsgType == 53) {
                println!("MsgType :{}", body_head.MsgType);
                println!("MsgSize :{}", body_head.MsgSize);

                let filler_nums = (pkt_size - 12) / 24;
                println!(" filler_nums {}", filler_nums);
                let body_data = LevelPrice::new(body_pack, filler_nums as u8);
                println!("{}", body_data);
            } else if body_head.MsgType == 62 {
                println!("MsgType :{}", body_head.MsgType);
            } else if (body_head.MsgType == 40) {
                let body_data = NominalPrice::new(body_pack);
                println!("else MsgType :{}", body_head.MsgType);
                println!("{}", body_data.SecurityCode);
                println!("{}", body_data.NominalPrice);
            } else if (body_head.MsgType == 13) {
                let body_data = LiquidityProvider::write_bytes(body_pack);
                println!("else MsgType :{}", body_head.MsgType);
                println!("MsgSize :{}", body_head.MsgSize);
                // println!("{}",body_data. )
                println!("{}", body_data.SecurityCode);
                println!("{}", body_data.NoLiquidityProviders);
                println!("{}", body_data.LPBrokerNumber);
            } else if (body_head.MsgType == 71) {
                let body_data = IndexData::write_bytes(body_pack);
                println!("else MsgType :{}", body_head.MsgType);
                println!("IndexCode :{}", body_data.IndexCode);
                println!("IndexValue :{}", body_data.IndexValue);
            } else if (body_head.MsgType == 54) {
                let body_data = BrokerQueue::write_bytes(body_pack);
                println!("else MsgType :{}", body_head.MsgType);
                println!("{}", body_data.SecurityCode);
                println!("{}", body_data.ItemCount);
                println!("{}", body_data.Side);
            } else {
                println!("error MsgType :{}", body_head.MsgType);
            }

            let (_, body) = body.split_at(pkt_size as usize);
            if (body.len()) > 0 {
                pack_end_flag = false;
            }
        }
    }
}

// let head = Head::new(buff.to_vec());
// println!( "{}", head.MsgSize);
// println!( "{}", head.MsgType);

//     while true {
//         let mut buff = [0; 4096];
//         let _ = stream.read(&mut buff); // ignore here too
//                                         // println!("{:?}",buff);

//         let head = Head::new(buff.to_vec());

//         while true {
//             let mut len = buff_status;
//             let mut buffer = ByteBuffer::new();
//             save_data(buff.to_vec(), &mut buffer);

//             if (head.MsgSize > buff_status) {
//                 len += buff_status;
//                 let _ = stream.read(&mut buff);
//                 save_data(buff.to_vec(), &mut buffer);
//             } else {
//                 println!("msg size {}", head.MsgSize);
//                 println!("msg type {}", head.MsgType);
//                 let msg_type = head.MsgType;
//                 if (msg_type == 33) {
//                     // let mut body_buff = [0;4092];
//                     // let _ = stream.read(&mut body_buff );
//                     // println!("{:?}",body_buff);
//                     // let mut packet_buff = buff.to_vec().extend(body_buff.iter().cloned());
//                     // println!("{:?}",packet_buff);
//                     let addoddlotorder = AddOddLotOrder::new(buff.to_vec());
//                     println!("{:?}", addoddlotorder);
//                 } else if (msg_type == 70) {
//                     let indexdefinition = IndexDefinition::new(buff.to_vec());
//                     println!("{:?}", indexdefinition);
//                 } else if (msg_type == 40) {
//                     let nominalprice = NominalPrice::new(buff.to_vec());
//                     println!("{:?}", nominalprice.MsgSize);
//                 }
//                 println!("ms type {}", head.MsgType);
//                 break;
//             }
//         }

//         // println!("{:?}",buff);
//     }
// }
// the stream is closed here
