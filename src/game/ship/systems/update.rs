use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game::{
        components::Market,
        ship::components::{Ship, ShipState},
    },
    st_client,
};

pub(crate) fn update_ships(mut ships: Query<(&mut Transform, &Ship, &ShipState)>, markets_query: Query<&Market>) {
    for (mut transform, ship, ship_state) in ships.iter_mut() {
        transform.translation = ship.get_position();
        // dbg!(ship_state.state.clone());

        if ship_state.is_idle() || ship.is_in_transit() {
            return;
        }

        if ship_state.has_to_find_new_item_to_purchase() {
            // let (item, purchase_waypoint, sell_waypoint) =  find_new_item_to_purchase(markets_query.iter()
            return;
        }

        // if ship.has_cargo() || ship.is_in_orbit() {
        //     st_client::dock_ship(ship.symbol.as_str());
        // }

        // if
    }
}

fn find_new_item_to_purchase(markets: Vec<Market>) -> (String, String, String) {
    let mut max_price_difference = 0.;
    let mut item = String::new();
    let mut purchase_waypoint = String::new();
    let mut sell_waypoint = String::new();
    // let price_map = HashMap::new();
    for market in markets.iter() {
        let mut tmp_max_price_difference = 0.;
        let mut tmp_item = String::new();
        let mut tmp_purchase_waypoint = String::new();
        let mut tmp_sell_waypoint = String::new();
        market.trade_goods.iter().for_each(|trade_good| {
            let mut lowest_purchase = trade_good.purchase_price;
            let mut highest_sell = trade_good.sell_price;
            tmp_item = trade_good.symbol.clone();

            for inner_market in markets.iter() {
                inner_market.trade_goods.iter().for_each(|inner_trade_good| {
                    if inner_trade_good.purchase_price < lowest_purchase {
                        lowest_purchase = inner_trade_good.purchase_price;
                        tmp_purchase_waypoint = inner_market.symbol.clone();
                    }
                    if inner_trade_good.sell_price > highest_sell {
                        highest_sell = inner_trade_good.sell_price;
                        tmp_sell_waypoint = inner_market.symbol.clone();
                    }
                });
            }
        });
    }

    (item, purchase_waypoint, sell_waypoint)
}


#[cfg(test)]
mod tests {
    use crate::game::{components::Market, ship::systems::update::find_new_item_to_purchase};



    #[test]
    fn test_markets() {
        let markets = vec![get_market_one(), get_market_two(), get_market_three()];

        find_new_item_to_purchase(markets);
        
        
        assert_eq!(2 + 2, 4);
    }

    fn get_market_three() -> Market {
        let market = r#"{
            "symbol": "X1-QB20-57458X",
            "imports": [],
            "exports": [],
            "transactions": [],
            "trade_goods": [
              {
                "symbol": "ICE_WATER",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 17.0,
                "sell_price": 13.0
              },
              {
                "symbol": "FOOD",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 242.0,
                "sell_price": 238.0
              },
              {
                "symbol": "MACHINERY",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 545.0,
                "sell_price": 534.0
              },
              {
                "symbol": "ELECTRONICS",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 506.0,
                "sell_price": 495.0
              },
              {
                "symbol": "PLASTICS",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 263.0,
                "sell_price": 257.0
              },
              {
                "symbol": "FUEL",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 122.0,
                "sell_price": 118.0
              },
              {
                "symbol": "MEDICINE",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 566.0,
                "sell_price": 554.0
              },
              {
                "symbol": "DRUGS",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 343.0,
                "sell_price": 336.0
              },
              {
                "symbol": "CLOTHING",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 283.0,
                "sell_price": 277.0
              },
              {
                "symbol": "EQUIPMENT",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 525.0,
                "sell_price": 514.0
              },
              {
                "symbol": "FABRICS",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 77.0,
                "sell_price": 73.0
              }
            ]
          }"#;
        serde_json::from_str::<Market>(market).unwrap()
    }
    fn get_market_two() -> Market {
        let market = r#"{
            "symbol": "X1-QB20-13975F",
            "imports": [],
            "exports": [],
            "transactions": [],
            "trade_goods": [
              {
                "symbol": "ICE_WATER",
                "trade_volume": 100000,
                "supply": "MODERATE",
                "purchase_price": 17.0,
                "sell_price": 13.0
              },
              {
                "symbol": "AMMONIA_ICE",
                "trade_volume": 100000,
                "supply": "MODERATE",
                "purchase_price": 42.0,
                "sell_price": 38.0
              },
              {
                "symbol": "QUARTZ_SAND",
                "trade_volume": 100000,
                "supply": "MODERATE",
                "purchase_price": 22.0,
                "sell_price": 18.0
              },
              {
                "symbol": "SILICON_CRYSTALS",
                "trade_volume": 100000,
                "supply": "MODERATE",
                "purchase_price": 37.0,
                "sell_price": 33.0
              },
              {
                "symbol": "IRON_ORE",
                "trade_volume": 10000,
                "supply": "MODERATE",
                "purchase_price": 47.0,
                "sell_price": 43.0
              },
              {
                "symbol": "COPPER_ORE",
                "trade_volume": 10000,
                "supply": "ABUNDANT",
                "purchase_price": 57.0,
                "sell_price": 53.0
              },
              {
                "symbol": "ALUMINUM_ORE",
                "trade_volume": 10000,
                "supply": "MODERATE",
                "purchase_price": 52.0,
                "sell_price": 48.0
              },
              {
                "symbol": "FUEL",
                "trade_volume": 10000,
                "supply": "MODERATE",
                "purchase_price": 122.0,
                "sell_price": 118.0
              },
              {
                "symbol": "PRECIOUS_STONES",
                "trade_volume": 1000,
                "supply": "ABUNDANT",
                "purchase_price": 89.0,
                "sell_price": 82.0
              },
              {
                "symbol": "DIAMONDS",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 465.0,
                "sell_price": 455.0
              }
            ]
          }"#;
        serde_json::from_str::<Market>(market).unwrap()
    }
    
    fn get_market_one() -> Market {
        let market_1 = r#"{
            "symbol": "X1-QB20-61050B",
            "imports": [],
            "exports": [],
            "transactions": [],     
            "trade_goods": [
              {
                "symbol": "FOOD",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 518.0,
                "sell_price": 255.0
              },
              {
                "symbol": "ELECTRONICS",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 1048.0,
                "sell_price": 519.0
              },
              {
                "symbol": "GRAVITON_EMITTERS",
                "trade_volume": 100,
                "supply": "MODERATE",
                "purchase_price": 2860.0,
                "sell_price": 1415.0
              },
              {
                "symbol": "FUEL",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 252.0,
                "sell_price": 125.0
              },
              {
                "symbol": "ICE_WATER",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 32.0,
                "sell_price": 16.0
              },
              {
                "symbol": "EQUIPMENT",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 1096.0,
                "sell_price": 543.0
              },
              {
                "symbol": "PLASTICS",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 558.0,
                "sell_price": 274.0
              },
              {
                "symbol": "CLOTHING",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 602.0,
                "sell_price": 296.0
              },
              {
                "symbol": "MACHINERY",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 1138.0,
                "sell_price": 563.0
              },
              {
                "symbol": "MEDICINE",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 1178.0,
                "sell_price": 583.0
              },
              {
                "symbol": "ADVANCED_CIRCUITRY",
                "trade_volume": 1000,
                "supply": "LIMITED",
                "purchase_price": 6084.0,
                "sell_price": 3000.0
              },
              {
                "symbol": "DRUGS",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 714.0,
                "sell_price": 353.0
              },
              {
                "symbol": "FABRICS",
                "trade_volume": 1000,
                "supply": "MODERATE",
                "purchase_price": 158.0,
                "sell_price": 79.0
              }
            ]
          }"#;

          serde_json::from_str::<Market>(market_1).unwrap()
        
    }

    
}