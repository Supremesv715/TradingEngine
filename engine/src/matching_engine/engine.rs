use rust_decimal::prelude::*;


use crate::{matching_engine::orderbook::{self, Order}, Orderbook};
use std::collections::HashMap;
#[derive(Debug,Eq,PartialEq,Hash,Clone)]
pub struct TradingPair{
    base:String,
    quote:String
}
impl TradingPair{
    pub fn new(base: String,quote:String)->TradingPair{
        TradingPair{base,quote}
    }
    pub fn to_string(self)->String{
        format!("{}_{}",self.base,self.quote)

    }
}
pub struct Matchingengine{
    orderbook:HashMap<TradingPair,Orderbook>,

}
impl Matchingengine{
    pub fn new()-> Matchingengine{
        Matchingengine{
            orderbook: HashMap::new(),
        }
    }
    pub fn add_new_market(&mut self,pair: TradingPair){
        self.orderbook.insert(pair.clone(),Orderbook::new());
        println!("opening new orderbook for market{:?}",pair.to_string());
    }
    pub fn place_limit_order(&mut self,pair: TradingPair,price:Decimal,order:Order)-> Result<(),String>{
        match self.orderbook.get_mut(&pair){
            Some(orderbook)=>{orderbook.add_limit_order(price,order);
                println!("place limit order at price level {}",price);
            Ok(())
        }
            None=>Err(format!(
                "the orderbook for the given trading pair ({}) does not exist",
                pair.to_string()
            )),
        }
    }
}



