use bevy::prelude::*;

use crate::{
    game::{components::Market, ship::components::Ship},
    st_client,
    ui::hud::components::ErrorText,
};

#[derive(Event)]
pub(crate) struct GetMarketAtShipLocationEvent(pub(crate) Entity);

pub(super) fn handle_get_market_clicked_for_ship_event(
    mut commands: Commands, mut events: EventReader<GetMarketAtShipLocationEvent>, mut ships: Query<&mut Ship>,
    existing_markets_query: Query<(Entity, &Market), With<Market>>, mut error_text: Query<&mut Text, With<ErrorText>>,

) {
    for my_event in events.iter() {
        let mut ship = ships.get_mut(my_event.0).unwrap();

        let market_details = st_client::get_market_data(&mut ship);
        match market_details {
            Ok(market_data) => {
                for (existing_market_entity, existing_market) in existing_markets_query.iter() {
                    if existing_market.symbol == market_data.symbol {
                        commands.entity(existing_market_entity).despawn_recursive();
                        break;
                    }
                }
                tracing::trace!("market data: {}", serde_json::to_string_pretty(&market_data).unwrap());
                let market_symbol = market_data.symbol.clone();
                commands.spawn((market_data, Name::new(format!("Market {}", market_symbol))));
            }
            Err(e) => {
                tracing::error!("{:?}", e);
                error_text.single_mut().sections[0].value = format!("Error: Unable to read market data {e}.").to_string();
            }
        }
    }
}
