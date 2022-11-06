use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SCategory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub image: String,
    pub created_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SProduct {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub sale_price: u32,
    pub actual_price: u32,
    pub description: String,
    pub in_stock: u16,
    pub image_url: String,
    pub is_trending: bool,
    pub category: ObjectId,
    pub created_at: DateTime,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub profile_pic: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wishlist: Option<Vec<ObjectId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SCart {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_id: ObjectId,
    pub quantity: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ObjectId>,
}

#[derive(Serialize, Deserialize)]
pub struct SAddress {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub pin_code: u32,
    pub phone_no: u64,
    pub default: bool,
    pub user: ObjectId,
    pub is_deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SOrder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_id: ObjectId,
    pub user: ObjectId,
    pub address: ObjectId,
    pub price: u32,
    pub payment_method: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct SLogin {
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct PaginationPayload {
    pub page: u16,
    pub limit: u16,
}


#[derive(Serialize)]
pub struct Meta {
    pub total: u16,
    pub page: u16,
    pub limit: u16,
}


#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}


#[derive(Serialize)]
pub struct LoginRes {
    pub token: String,
    pub user: SUser,
}


#[derive(Deserialize)]
pub struct IDReq {
    pub id: String
}

#[derive(Serialize)]
pub struct MinRes {
    pub status: bool,
    pub message: String,
}
