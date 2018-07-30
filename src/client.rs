use std::time::Duration;
use std::fmt::Write as FmtWrite;
use std::io::Error;

use url::percent_encoding::{percent_encode, QUERY_ENCODE_SET};
use actix_web::{client, Body, Binary, HttpMessage};
use actix_web::http::Method;
use std::collections::HashMap;
use futures::future::{Future, ok, err};
use actix_web::client::{ClientResponse, ClientConnector, Connect};

pub struct MyError<'a>(&'a str);

//https://docs.rs/futures/0.1.23/futures/

pub fn get<'a>(url: &'a str, params: &HashMap<&str, &str>) -> Box<Future<Item=String, Error=MyError<'a>>> {
    req(url, params, Method::GET)
}

pub fn post<'a>(url: &'a str, params: &HashMap<&str, &str>) -> Box<Future<Item=String, Error=MyError<'a>>> {
    req(url, params, Method::POST)
}

fn req<'a>(url: &'a str, params: &HashMap<&str, &str>, method: Method) -> Box<Future<Item=String, Error=MyError<'a>>> {
    let mut combined_params = String::with_capacity(1024);
    for (key, value) in params {
        println!("{} / {}", key, value);
        let value = percent_encode(value.as_bytes(), QUERY_ENCODE_SET).to_string();
        let _ = write!(&mut combined_params, "{}={}&", key, value);
    }

//    println!("combined_params = {}", combined_params);

    let mut builder = client::ClientRequest::build();
    builder.uri(url);
    builder.timeout(Duration::new(6, 0));
    if method == Method::POST {
        builder.method(Method::POST);
    } else {
        builder.method(Method::GET);
    }

    let result = builder
        .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
        .header("User-Agent", "Anti-Lazy V1.0")
        .header("Accept", "application/json")
//        .body(Body::Binary())
//        .finish();
        .body(combined_params);
    let c = match result {
        Ok(c) => c,
        Err(e) => {
            println!("request err1 = {}", e);
            return Box::new(err(MyError("Error::InvalidUrlArgument")))
        },
    };

//    c.send().from_err().and_then(|res| {
//        //todo
//    });
    let res = c.send()
        .map_err(|e| {
            println!("request err2 = {}", e);
            MyError("Error::InvalidUrlArgument")
        })
        .and_then(|response: ClientResponse| {
            println!("Response: {:?}", response);
            response.body().then(|result| {
                match result {
                    Ok(b) => ok(String::from_utf8(b.to_vec())),
                    Err(e) => err(MyError("Error::InvalidUrlArgument")),
                }
            })
        });

    let ret = res.from_err().and_then(|result| {
        match result {
            Ok(s) => ok(s),
            Err(e) => ok("".to_string()),
        }
    });

    Box::new(ret)
}