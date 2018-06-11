use serde_json::Error;

use packet::*;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Price {
    marketCode: String,
    lastPrice: f32,
    timestamp: u32,
    symbol: String,
    stop: u8,
    openPrice: f32,
    volume: u32,
    date: String,
    price: f32,
    closePrice: f32,
    name: String,
    lowPrice: f32,
    highPrice: f32,
    eps: f32,
    amount: u64,
    issueCap: u64,
    lotSize: u32,
    time: String,
    instrumentType: String,
}

// trait Foo{
//     fn to_json() ->Result<String, Error> {

//     }
// }

impl Price {
    pub fn new(pack_Price: NominalPrice) ->Self{
        let mut price = Self::other();
        price.price = pack_Price.NominalPrice as f32 ;
        // price.symbol =  ;
        return price
    }

    pub fn to_json(self) -> Result<String, Error>{
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }

    fn other() -> Self {
        Price {
            marketCode: String::from("MAIN"),
            lastPrice: 429.2,
            timestamp: 1528436705969,
            symbol: String::new(),
            stop: 1,
            openPrice: 429.2,
            volume: 0,
            date: String::from("2018-06-08"),
            price: 0.0,
            closePrice: 429.2,
            name: String::new(),
            lowPrice: 0.0,
            highPrice: 0.0,
            eps: 0.0,
            amount: 0,
            issueCap: 0,
            lotSize: 0,
            time: String::from("13: 45: 05.969"),
            instrumentType: String::from("EQTY"),
        }
    }
}

// "{
//     \"marketCode\": \"MAIN\",
//     \"lastPrice\": 429.2,
//     \"timestamp\": 1528436705969,
//     \"symbol\": \"00700.HK\",
//     \"stop\": 0,
//     \"openPrice\": 426.6,
//     \"volume\": 16217599,
//     \"date\": \"2018-06-08\",
//     \"price\": 417.0,
//     \"closePrice\": 0,
//     \"name\": \"\\u817e\\u8baf\\u63a7\\u80a1\",
//     \"lowPrice\": 415.4,
//     \"highPrice\": 426.6,
//     \"eps\": \"3.225\",
//     \"amount\": 6.813397e+09,
//     \"issueCap\": \"9403923992\",
//     \"lotSize\": 100,
//     \"time\": \"13: 45: 05.969\",
//     \"instrumentType\": \"EQTY\"
// }"
