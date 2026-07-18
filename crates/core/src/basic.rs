use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Display, Deserialize, Serialize)]
pub enum RequestMethod {
    #[strum(serialize = "POST")]
    POST,
    #[strum(serialize = "GET")]
    GET,
}
