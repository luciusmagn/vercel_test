use http::StatusCode;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

use horrorshow::helper::doctype;
use horrorshow::html;
use horrorshow::prelude::*;

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let document = html! {
        : doctype::HTML;
        html {
            head {
                // Use a variable
                title : "Hello, world!";
            }
            body {
                // attributes
                h1(id="heading", class="title") : my_title;
                p {
                    // Insert escaped text
                    : "Hello! This is <html />";
                }
                p {
                    // Insert raw text (unescaped)
                    : Raw("Let's <i>count</i> to 10!");
                }
                ol(id="count") {
                    // You can embed for loops, while loops, and if statements.
                    @ for i in 0..10 {
                        li(first? = (i == 0), class="item") {
                            // Format some text.
                            : format_args!("{}", i+1)
                        }
                    }
                }
                // You need semi-colons for tags without children.
                br; br;
                p {
                    // You can also embed closures.
                    |tmpl| {
                        tmpl << "Easy!";
                    }
                }
            }
        }
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("Hello World")
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
