#![allow(dead_code)]

use std::fmt::Display;

use bevy::{prelude::{Component, ReflectComponent}, reflect::Reflect};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub(crate) struct Market {
    pub(crate) symbol: String,
    imports: Vec<ImportExport>,
    exports: Vec<ImportExport>,
    transactions: Vec<Transaction>,
    #[serde(alias = "tradeGoods")]
    pub(crate) trade_goods: Vec<TradeGood>,
}

impl Market {
    pub(crate) fn display_exports(&self) -> String {
        let mut exports = String::new();
        for export in &self.exports {
            exports.push_str(&format!("{}\n", export));
        }
        exports
    }
    pub(crate) fn display_imports(&self) -> String {
        let mut exports = String::new();
        for export in &self.imports {
            exports.push_str(&format!("{}\n", export));
        }
        exports
    }
    pub(crate) fn display_trade_goods(&self) -> String {
        let mut exports = String::new();
        for export in &self.trade_goods {
            exports.push_str(&format!("{}\n", export));
        }
        exports
    }

    // pub(crate) fn get_trade_goods(&self) -> Vec<TradeGood> {
    //     self.trade_goods.cl
    // }
}

#[derive(Deserialize, Serialize, Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub(crate) struct ImportExport {
    pub(crate) symbol: String,
    pub(crate) name: String,
    pub(crate) description: String,
}

impl Display for ImportExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize, Serialize, Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub(crate) struct Transaction {
    #[serde(alias = "waypointSymbol")]
    pub(crate) waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub(crate) ship_symbol: String,
    #[serde(alias = "tradeSymbol")]
    pub(crate) trade_symbol: String,
    #[serde(alias = "type")]
    pub(crate) transaction_type: String,
    pub(crate) units: i32,
    #[serde(alias = "pricePerUnit")]
    pub(crate) price_per_unit: f32,
    #[serde(alias = "totalPrice")]
    pub(crate) total_price: f32,
    pub(crate) timestamp: String,
}

#[derive(Deserialize, Serialize, Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub(crate) struct TradeGood {
    pub(crate) symbol: String,
    #[serde(alias = "tradeVolume")]
    pub(crate) trade_volume: i32,
    pub(crate) supply: String,
    #[serde(alias = "purchasePrice")]
    pub(crate) purchase_price: f32,
    #[serde(alias = "sellPrice")]
    pub(crate) sell_price: f32,
}

// impl TradeGood {
//     pub(crate) get_margin
// }

impl Display for TradeGood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} / {}", self.symbol, self.purchase_price, self.sell_price)
    }
}