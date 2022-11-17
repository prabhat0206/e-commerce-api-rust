use mongodb::{Client, Collection};
use crate::serializers::{SCategory, SProduct, SAddress, SCart, SOrder, SUser};


const MONGO_URI: &str =
    "mongodb+srv://localhost/ecomm";

pub struct Database {
    pub user_col: Collection<SUser>,
    pub address_col: Collection<SAddress>,
    pub order_col: Collection<SOrder>,
    pub cart_col: Collection<SCart>,
    pub category_col: Collection<SCategory>,
    pub product_col: Collection<SProduct>,
}

impl Database {
    pub async fn init() -> Self {
        let client = Client::with_uri_str(MONGO_URI).await.unwrap();
        let db = client.database("ecomm");
        let user_col = db.collection("user");
        let address_col = db.collection("address");
        let order_col = db.collection("order");
        let cart_col = db.collection("cart");
        let category_col = db.collection("category");
        let product_col = db.collection("product");

        Self {
            user_col,
            address_col,
            order_col,
            cart_col,
            category_col,
            product_col,
        }
    }
}
