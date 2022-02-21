use http::StatusCode;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

use horrorshow::helper::doctype;
use horrorshow::html;

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let document = html! {
        : doctype::HTML;
        html {
            head {
                // Use a variable
                title : "Hello, world!";
                link(rel="preload", href="Joan-Regular.woff2", as="font", type="font/woff2");
                link(rel="stylesheet", href="https://fonts.googleapis.com/css?family=Roboto:300,300italic,700,700italic");
                link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.css");
                link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/milligram/1.4.1/milligram.css");
                style {
                    : include_str!("../static/font-styles.css")
                }
                style {
                    : "
                        body {
                            font-family: 'Joan';
                            color: black;
                        }
                        #heading {
                            text-align: center;
                            width: 100%;
                            padding: 10rem;
                            font-size: 20rem;
                            font-weight: 300;
                        }
                    "
                }
            }
            body(class="container") {
                header(class="row") {
                    h1(id="heading") : "Tau";
                }
                p {
                    : "Hello! This is <html />";
                }
            }
        }
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(document.to_string())
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(|r| handler(r)))
}
