use crate::{
    database::Database,
    serializers::{IDReq, Meta, PaginatedResponse, MinRes},
    utilities::get_user_from_header,
};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};

pub async fn get_wishlist_items(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
    let user = get_user_from_header(req.clone(), db.clone()).await;

    let pipeline = vec![doc! {
        "_id": doc! {
            "$in": user.wishlist.clone(),
        }
    }];

    let mut product_cursor = db.product_col.aggregate(pipeline, None).await.unwrap();
    let mut products: Vec<Document> = vec![];
    while let Some(product) = product_cursor.try_next().await.unwrap() {
        products.push(product)
    }

    let meta = Meta {
        total: products.clone().len() as u16,
        page: 0,
        limit: 0,
    };

    let res = PaginatedResponse {
        data: products,
        meta,
    };

    return HttpResponse::Ok().json(res);
}

pub async fn add_to_wishlist(
    db: web::Data<Database>,
    body: web::Json<IDReq>,
    req: HttpRequest,
) -> HttpResponse {
    if body.id.is_empty() {
        return HttpResponse::BadRequest().body("Invaild id");
    }

    let product_id = ObjectId::parse_str(body.id.clone()).unwrap();

    let user = get_user_from_header(req.clone(), db.clone()).await;

    let product = db
        .product_col
        .find_one(
            Some(doc! {
                "_id": product_id.clone()
            }),
            None,
        )
        .await
        .unwrap();

    if product.is_none() {
        return HttpResponse::Forbidden().body("product not found");
    }

    if user.clone().wishlist.unwrap().contains(&product_id) {
        return HttpResponse::BadRequest().body("product already exits");
    }

    let mut new_wishlist = vec![product_id];
    new_wishlist.append(&mut user.wishlist.unwrap());

    db.user_col
        .update_one(
            doc! {
               "_id": user.id
            },
            doc! {
                "$set": doc! {
                    "wishlist": new_wishlist
                }
            },
            None,
        )
        .await
        .unwrap();

    return HttpResponse::Ok().json(MinRes {
        status: true,
        message: "product added to wishlist".to_string(),
    });
}

pub async fn delete_from_wishlist(
    db: web::Data<Database>,
    req: HttpRequest,
    body: web::Json<IDReq>,
) -> HttpResponse {
    if body.id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid id");
    }

    let product_id = ObjectId::parse_str(body.id.clone()).unwrap();
    let user = get_user_from_header(req.clone(), db.clone()).await;

    if !user.clone().wishlist.unwrap().contains(&product_id.clone()) {
        return HttpResponse::BadRequest().body("product not found in wishlist");
    }

    let mut new_wishlist = Vec::new();

    while let Some(id) = user.clone().wishlist.unwrap().pop() {
        if id != product_id {
            new_wishlist.push(id);
        }
    }

    db.user_col
        .update_one(
            doc! {
                "_id": user.id
            },
            doc! {
                "$set": doc! {
                    "wishlist": new_wishlist
                }
            },
            None,
        )
        .await
        .unwrap();

    return HttpResponse::Ok().json(MinRes {
        status: true,
        message: "product deleted from wishlist".to_string(),
    });
}
