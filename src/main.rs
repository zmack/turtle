use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, header::HeaderValue};
use hyper::header::CONTENT_TYPE;
use hyper::service::{make_service_fn, service_fn};
use tokio::time::sleep;
use std::time::Duration;

const JSON_RESPONSE: &str = r#"
{
  "Name": "San Francisco Intl",
  "City": "San Francisco",
  "State": "CA",
  "ICAO": "KSFO",
  "IATA": "SFO",
  "SupportedAirport": true,
  "Delay": false,
  "DelayCount": 0,
  "Status": [
    {
      "Reason": "No known delays for this airport"
    }
  ],
  "Weather": {
    "Weather": [
      {
        "Temp": [
          "Partly Cloudy"
        ]
      }
    ],
    "Visibility": [
      10
    ],
    "Meta": [
      {
        "Credit": "NOAA's National Weather Service",
        "Url": "http://weather.gov/",
        "Updated": "Last Updated on May 22 2021, 8:56 pm PDT"
      }
    ],
    "Temp": [
      "54.0 F (12.2 C)"
    ],
    "Wind": [
      "West at 17.3"
    ]
  }
}
"#;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    sleep(Duration::new(1, 0)).await;
    let mut response = Response::new(JSON_RESPONSE.into());
    response.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
