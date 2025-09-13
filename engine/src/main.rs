
pub mod matching_engine;
use matching_engine::orderbook::{Order,Orderbook,BidOrAsk};
use matching_engine::engine::{TradingPair,Matchingengine};
use rust_decimal_macros::dec;
fn main(){
  
   let buy_orders_from_sarthak=Order::new( BidOrAsk::Bid,5.5);
   let buy_orders_from_rudra=Order::new( BidOrAsk::Bid,5.5);

   let mut orderbook=Orderbook::new();
   orderbook.add_limit_order(dec!(4.4), buy_orders_from_sarthak);
   orderbook.add_limit_order(dec!(4.4), buy_orders_from_rudra);
   let sell_order=Order::new (BidOrAsk::Ask,6.5);
   orderbook.add_limit_order(dec!(20.0),sell_order);
   
   
   let mut engine=Matchingengine::new();
 
   let pair=TradingPair::new("BTC".to_string(),"USD".to_string());
   engine.add_new_market(pair.clone());
   let buy_order=Order::new(BidOrAsk::Bid,6.5);
   
   engine.place_limit_order(pair, dec!(10.000), buy_order).unwrap();

}