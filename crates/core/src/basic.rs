use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Display, Deserialize, Serialize)]
pub enum RequestMethod {
    NULL,
    #[strum(serialize = "POST")]
    POST,
    #[strum(serialize = "GET")]
    GET,
    #[strum(serialize = "PUT")]
    PUT,
    #[strum(serialize = "DELETE")]
    DELETE,
    #[strum(serialize = "PATCH")]
    PATCH,
}
