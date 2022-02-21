use http::StatusCode;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

use horrorshow::helper::doctype;
use horrorshow::html;

use jss::prelude::*;

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let document = html! {
        : doctype::HTML;
        html {
            head {
                // Use a variable
                title : "mag.wiki";
                link(rel="preload", href="Joan-Regular.woff2", as="font", type="font/woff2");
                link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/milligram/1.4.1/milligram.css");
                style {
                    : include_str!("../static/font-styles.css")
                }
                style {
                    : jss!(
                        "body": {
                            font_family: "Joan",
                            color: "black",
                        },
                        "#heading": {
                            text_align: "center",
                            width: percent(100u8),
                            padding: rem(10u8),
                            font_size: rem(20u8),
                            font_weight: 300u16,
                        },
                        "nav a": {
                            display: "block",
                            text_align: "right",
                            color: "black",
                            font_size: rem(3u8),
                            font_weight: 300u16,
                            cursor: "pointer",
                            transition: ".3s",
                        },
                        "nav a::after": {
                            content: "' →'",
                            font_weight: 600u16,
                        },
                        "nav a:hover": {
                            color: "black",
                            font_weight: 700u16,
                        },
                        "article": {
                            margin_bottom: rem(2u8),
                        },
                        "article p": {
                            font_size: rem(2u8),
                            text_align: "justify",
                        },
                        "article h1": {
                            font_weight: 400u16,
                        },
                        "article span": {
                            color: "gray",
                            text_transform: "uppercase",
                        },
                        "a.more": {
                            font_weight: 500u16,
                            text_decoration: "underline",
                            color: "black",
                            display: "block",
                            cursor: "pointer",
                        }
                    )
                }
            }
            body(class="container") {
                header(class="row") {
                    h1(id="heading") : "mag.wiki";
                }
                div(class="row") {
                    nav(class="column column-20") {
                        a: "Home";
                        a: "Contacts";
                        a: "Warcrimes";
                        a: "Source";
                    }
                    section(class="column column-offset-10") {
                        article {
                            h1: "Hello, this is a bullshit article";
                            span: "22.02.2022 / Lukáš Hozda";
                            p {
                                : "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.                                "
                            }
                            a(class="more"): "Read more"
                        }
                        article {
                            h1: "Hello, this is a bullshit article";
                            span: "22.02.2022 / Lukáš Hozda";
                            p {
                                : "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.                                "
                            }
                            a(class="more"): "Read more"
                        }
                    }
                }
            }
        }
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(document.to_string())
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(|r| handler(r)))
}
