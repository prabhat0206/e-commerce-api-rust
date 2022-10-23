use crate::database::Database;
use crate::serializers::{LoginRes, Meta, PaginatedResponse, SLogin, SUser};
use crate::utilities::{generate_jwt, get_user_from_header};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use pwhash::bcrypt;

pub async fn signup(db: web::Data<Database>, body: web::Json<SUser>) -> HttpResponse {
    if body.name.is_empty()
        || body.email.is_empty()
        || body.phone.is_empty()
        || body.password.is_empty()
    {
        return HttpResponse::BadRequest().body("Invalid fields");
    }

    let check_email_exists = db
        .user_col
        .find_one(Some(doc! {"email": &body.email}), None)
        .await;

    print!("{:?}", check_email_exists);

    match check_email_exists {
        Ok(user) => {
            if user.is_some() {
                return HttpResponse::BadRequest().body("Email already exists");
            }
        }
        Err(_) => {}
    }

    let check_phone_exists = db
        .user_col
        .find_one(Some(doc! {"phone": &body.phone}), None)
        .await;

    match check_phone_exists {
        Ok(user) => {
            if user.is_some() {
                return HttpResponse::BadRequest().body("Phone number already exists");
            }
        }
        Err(_) => {}
    }

    let user = SUser {
        id: Some(ObjectId::new()),
        name: body.name.clone(),
        email: body.email.clone(),
        phone: body.phone.clone(),
        profile_pic: body.profile_pic.clone(),
        password: bcrypt::hash(&body.password).unwrap(),
        active: Some(true),
        wishlist: Some(vec![]),
        create_at: Some(chrono::Utc::now().to_string()),
    };

    let result = db.user_col.insert_one(user, None).await;

    match result {
        Ok(user) => {
            let user = user.inserted_id.as_object_id().unwrap();
            let user = db
                .user_col
                .find_one(Some(doc! {"_id": user}), None)
                .await
                .unwrap()
                .unwrap();
            let token = generate_jwt(user.clone());
            let res = LoginRes {
                token: token,
                user: user.clone(),
            };
            return HttpResponse::Ok().json(res);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn login(db: web::Data<Database>, body: web::Json<SLogin>) -> HttpResponse {
    if body.email.is_empty() || body.password.is_empty() {
        return HttpResponse::BadRequest().body("Invalid Request");
    }

    let user = db
        .user_col
        .find_one(Some(doc! {"email": &body.email}), None)
        .await;

    match user {
        Ok(user) => {
            if user.is_some() {
                let user = user.unwrap();
                if bcrypt::verify(&body.password, &user.password) {
                    let token = generate_jwt(user.clone());
                    let res = LoginRes {
                        token,
                        user: user.clone(),
                    };
                    return HttpResponse::Ok().json(res);
                }
            }
            return HttpResponse::BadRequest().body("Invalid credentials");
        }
        Err(_) => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn get_wishlist_items(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
    let user = get_user_from_header(req.clone(), db.clone()).await;
    let pipeline = vec![
        doc! {
            "$match": doc! {
                "_id": doc! {
                    "$in": user.wishlist.clone()
                }
            }
        },
        doc! {
            "$sort": doc! {
                "created_at": -1
            }
        },
    ];

    let mut products_cursor = db.product_col.aggregate(pipeline, None).await.unwrap();
    let mut products = Vec::new();
    while let Some(product) = products_cursor.try_next().await.unwrap() {
        products.push(product);
    }

    let size_of_products: u16 = products.len() as u16;

    let res = PaginatedResponse {
        data: products,
        meta: Meta {
            total: size_of_products,
            limit: 0,
            page: 0,
        },
    };

    return HttpResponse::Ok().json(res);
}

