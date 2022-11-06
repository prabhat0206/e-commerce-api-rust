use actix_web::web;
mod user;
mod product;
mod cart;
mod wishlist;
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
            .route("/cart", web::delete().to(cart::delete_item_from_cart).wrap(PermissionCheck))
            .route("/wishlist", web::get().to(wishlist::get_wishlist_items).wrap(PermissionCheck))
            .route("/wishlist", web::post().to(wishlist::add_to_wishlist).wrap(PermissionCheck))
            .route("/wishlist", web::delete().to(wishlist::delete_from_wishlist).wrap(PermissionCheck))
    );
}
