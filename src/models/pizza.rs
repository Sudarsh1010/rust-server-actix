use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[Validate(length(min = 1, message = "Pizza name is required"))]
    pub pizza_name: String,
}