use crate::st_client;

use super::components::Ships;



pub(crate) fn fetch_my_ships() -> Ships {
    let resp = st_client::send_get("https://api.spacetraders.io/v2/my/ships");
    let data_wrapper: st_client::GenericResponse<Ships> = serde_json::from_str(&resp.unwrap()).unwrap();
    data_wrapper.data
}
