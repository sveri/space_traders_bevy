#![allow(dead_code)]

use bevy::{prelude::{Component, ReflectComponent}, reflect::Reflect};
use serde::Deserialize;

#[derive(Deserialize, Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub(crate) struct Market {
    pub(crate) symbol: String,
    imports: Vec<ImportExport>,
    exports: Vec<ImportExport>,
    transactions: Vec<Transaction>,
    #[serde(alias = "tradeGoods")]
    trade_goods: Vec<TradeGood>,
}

#[derive(Deserialize, Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub(crate) struct ImportExport {
    pub(crate) symbol: String,
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Deserialize, Reflect, Component, Default, Debug)]
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

#[derive(Deserialize, Reflect, Component, Default, Debug)]
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