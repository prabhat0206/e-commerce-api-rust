use crate::database::Database;
use crate::serializers::{Meta, PaginatedResponse, PaginationPayload};
use actix_web::{web, HttpResponse};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};

pub async fn get_categories(db: web::Data<Database>) -> HttpResponse {
    let mut cursor = db.category_col.find(None, None).await.unwrap();
    let mut categories = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        categories.push(result);
    }
    return HttpResponse::Ok().json(categories);
}

pub async fn get_products(
    db: web::Data<Database>,
    query: web::Query<PaginationPayload>,
) -> HttpResponse {
    let start = (query.page - 1) * query.limit;
    let end = start + query.limit;
    let pipeline = vec![
        doc! {
            "$sort": doc! {
                "created_at": -1
            }
        },
        doc! {
            "$skip": start as f64
        },
        doc! {
            "$limit": end as f64
        },
    ];

    let mut cursor = db.product_col.aggregate(pipeline, None).await.unwrap();
    let mut products = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        products.push(result);
    }

    let meta = Meta {
        total: products.len() as u16,
        page: query.page,
        limit: query.limit,
    };
    let response = PaginatedResponse {
        data: products,
        meta,
    };
    return HttpResponse::Ok().json(response);
}

pub async fn get_product_by_category(
    db: web::Data<Database>,
    path: web::Path<String>,
    query: web::Query<PaginationPayload>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid request");
    }

    let start = (query.page - 1) * query.limit;
    let end = start + query.limit;

    let pipeline = vec![
        doc! {
            "$match": doc! {
                "category": ObjectId::parse_str(id).unwrap()
            }
        },
        doc! {
            "$sort": doc! {
                "created_at": -1
            }
        },
        doc! {
            "$skip": start as f64
        },
        doc! {
            "$limit": end as f64
        },
    ];

    let mut products_cursor = db.product_col.aggregate(pipeline, None).await.unwrap();

    let mut products = Vec::new();
    while let Some(result) = products_cursor.try_next().await.unwrap() {
        products.push(result);
    }

    let meta = Meta {
        total: products.len() as u16,
        limit: query.limit,
        page: query.page,
    };

    let res = PaginatedResponse {
            data: products,
            meta,
        };
        return HttpResponse::Ok().json(res);
}
