#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate chrono;

#[get("/")]
fn index() -> &'static str {
    let hello = "Hello, world!";

    hello
}

#[get("/date")]
fn page_a() -> String {
    use chrono::Utc;
    let now = Utc::now().format("%Y年%m月%d日 %H時%M分%S秒 %Z").to_string();
    return now;
}



fn main() {
    rocket::ignite().mount("/", routes![index,page_a]).launch();
}