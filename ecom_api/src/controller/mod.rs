use actix_web::web;
mod user;
mod product;
mod cart;
use crate::middleware::PermissionCheck;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rest")
            .route("/signup", web::post().to(user::signup))
            .route("/login", web::post().to(user::login))
            .route("/categories", web::get().to(product::get_categories))
            .route("/products", web::get().to(product::get_products))
            .route("/product/{id}", web::get().to(product::get_product_by_category))
            .route("/wishlist", web::get().to(user::get_wishlist_items).wrap(PermissionCheck))
            .route("/cart", web::get().to(cart::get_cart_items).wrap(PermissionCheck))
            .route("/cart", web::post().to(cart::add_to_cart).wrap(PermissionCheck))
    );
}
