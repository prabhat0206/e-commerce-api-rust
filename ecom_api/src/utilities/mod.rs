use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::{serializers::SUser, database::Database};
use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, web};
use mongodb::bson::{oid::ObjectId, doc};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub fn generate_jwt(user: SUser) -> String {

    let claims = Claims {
        sub: user.id.unwrap().to_string(),
        role: "user".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ).unwrap();

    return token
}


pub fn verify_jwt(token: String) ->  String {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    if token_data.is_err() {
        return "Invalid token".to_string();
    }

    let token_data = token_data.unwrap();
    return token_data.claims.sub.clone();
}

pub async fn get_user_from_header(req: HttpRequest, db: web::Data<Database>) -> SUser {
    let user_id = req.headers().get("userid").unwrap().to_str().unwrap();
    let user_id = ObjectId::parse_str(&user_id).unwrap();
    let user = db.user_col.find_one(Some(doc! {"_id": user_id}), None).await.unwrap().unwrap();
    return user;
}


pub fn get_user_id(req: HttpRequest) -> String {
    return req.headers().get("userid").unwrap().to_str().unwrap().to_string();
}

