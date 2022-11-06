use crate::{
    database::Database,
    serializers::{IDReq, Meta, MinRes, PaginatedResponse, SCart},
    utilities::{get_user_from_header, get_user_id},
};

use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};

pub async fn get_cart_items(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
    let user_id = get_user_id(req.clone());

    let mut cart_cursor = db
        .cart_col
        .find(
            Some(doc! {"user": ObjectId::parse_str(user_id).unwrap()}),
            None,
        )
        .await
        .unwrap();

    let mut cart_items = Vec::new();
    while let Some(item) = cart_cursor.try_next().await.unwrap() {
        cart_items.push(item);
    }

    let meta = Meta {
        total: cart_items.len() as u16,
        limit: 0,
        page: 0,
    };

    let res = PaginatedResponse {
        data: cart_items,
        meta,
    };

    return HttpResponse::Ok().json(res);
}

pub async fn add_to_cart(
    db: web::Data<Database>,
    body: web::Json<SCart>,
    req: HttpRequest,
) -> HttpResponse {
    let user = get_user_from_header(req.clone(), db.clone()).await;

    if body.product_id.to_string().is_empty() || body.quantity as u32 == 0 {
        return HttpResponse::BadRequest().json("Invalid product id or quantity");
    } else {
        let cart = db.cart_col.find_one(
            Some(doc! {"user": user.id, "product_id": ObjectId::parse_str(body.product_id.clone().to_string()).unwrap()}),
            None,
        ).await.unwrap();

        if cart.is_some() {
            let cart = cart.unwrap();
            let quantity = body.quantity;

            db.cart_col
                .update_one(
                    doc! {"_id": cart.id.clone()},
                    doc! {"$set": {"quantity": quantity }},
                    None,
                )
                .await
                .unwrap();

            return HttpResponse::Ok().json("Cart updated");
        }

        let cart_item = SCart {
            id: Some(ObjectId::new()),
            user: user.id,
            product_id: ObjectId::parse_str(body.product_id.to_string().as_str()).unwrap(),
            quantity: body.quantity,
        };

        db.cart_col.insert_one(cart_item, None).await.unwrap();

        return HttpResponse::Ok().json(MinRes {
            status: true,
            message: "Added to cart".to_string(),
        })
    }
}

pub async fn delete_item_from_cart(
    db: web::Data<Database>,
    body: web::Json<IDReq>,
    req: HttpRequest,
) -> HttpResponse {
    if body.id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid id");
    }

    let user_id = get_user_id(req.clone());

    db.cart_col.delete_one(
        doc! {"product_id": ObjectId::parse_str(body.id.clone()).unwrap(), "user": ObjectId::parse_str(user_id).unwrap()},
        None,
    ).await.unwrap();

    return HttpResponse::Ok().json(MinRes {
        status: true,
        message: "Item deleted from cart".to_string(),
    });
}
