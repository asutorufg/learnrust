use hyper::{service::Service, Uri};
use hyper::{Body, Response};
use std::fmt::Debug;
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{self, Poll},
};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct ConnectError {
    msg: String,
}
impl ConnectError {
    fn new(msg: String) -> ConnectError {
        ConnectError { msg }
    }
}
impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl std::error::Error for ConnectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
#[tokio::main]
pub async fn connect() -> Result<(), ConnectError> {
    let ts = tokio::net::TcpStream::connect("127.0.0.1:8188")
        .await
        .expect("std stream to tokio stream failed");

    loop {
        ts.writable().await.expect("wait stream writable failed");
        match ts.try_write(b"CONNECT www.google.com:443 HTTP/1.1\r\n\r\n") {
            Ok(_) => {
                println!("write connect request successful");
                break;
            }
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(err) => return Err(ConnectError::new(err.to_string())),
        }
    }

    let mut buffer = vec![0; 4096];

    loop {
        ts.readable().await.expect("waite stream readable failed");
        match ts.try_read(&mut buffer) {
            Ok(n) => {
                println!("read connect request success, size: {}", n);
                break;
            }
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                continue;
            }

            Err(err) => return Err(ConnectError::new(err.to_string())),
        }
    }

    println!("{}", std::str::from_utf8_mut(&mut buffer).expect("msg"));

    let cx = native_tls::TlsConnector::builder()
        .build()
        .expect("build tls connector failed");
    let cx = tokio_native_tls::TlsConnector::from(cx);
    let stream = cx
        .connect("www.google.com", ts)
        .await
        .expect("tls connect failed");

    let (mut sender, connection) = hyper::client::conn::handshake(stream)
        .await
        .expect("handshake custom stream failed");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let req = hyper::Request::get("https://www.google.com/".parse::<hyper::Uri>().unwrap())
        .header("Host", "www.google.com")
        .header("User-Agent", "curl/v2.4.1")
        .body(Body::empty())
        .expect("create request failed");

    let resp: Response<Body>;
    match sender.send_request(req).await {
        Ok(r) => resp = r,
        Err(e) => panic!("{}", e),
    }

    let body_bytes = hyper::body::to_bytes(resp.into_body())
        .await
        .expect("body to bytes failed");

    let body = String::from_utf8(body_bytes.to_vec()).expect("msg");
    println!("{}", body);
    Ok(())
}

#[tokio::main]
pub async fn connect2() -> Result<(), ConnectError> {
    let ts = tokio::net::TcpStream::connect("127.0.0.1:50051")
        .await
        .expect("connect failed");

    let (mut sender, connection) = hyper::client::conn::Builder::new()
        .handshake::<tokio::net::TcpStream, hyper::Body>(ts)
        .await
        .expect("handshake custom stream failed");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let req = hyper::Request::get("http://127.0.0.1:50051/".parse::<hyper::Uri>().unwrap())
        .header("Host", "127.0.0.1:50051")
        .body(Body::empty())
        .expect("create request failed");

    let resp = sender
        .send_request(req)
        .await
        .expect("send get request failed");

    let body_bytes = hyper::body::to_bytes(resp.into_body())
        .await
        .expect("body to bytes failed");

    let body = String::from_utf8(body_bytes.to_vec()).expect("msg");
    println!("{}", body);
    Ok(())
}

#[derive(Clone)]
struct LocalConnector;

impl Service<Uri> for LocalConnector {
    type Response = TcpStream;
    type Error = std::io::Error;
    // We can't "name" an `async` generated future.
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        // This connector is always ready, but others might not be.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Uri) -> Self::Future {
        let ts = TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], 1337)));

        Box::pin(ts)
    }
}
#[tokio::main]
pub async fn connect3() -> Result<(), ConnectError> {
    let ts = tokio::net::TcpStream::connect("127.0.0.1:8188")
        .await
        .unwrap();

    let (mut sender, connection) = hyper::client::conn::handshake(ts)
        .await
        .expect("handshake custom stream failed");

    // let parts = connection.without_shutdown().await.expect("msg");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let creq = hyper::Request::connect("http://127.0.0.1:50051".parse::<hyper::Uri>().unwrap())
        .body(Body::empty())
        .expect("create creq failed");

    let cresp = sender.send_request(creq).await.expect("send creq failed");
    println!("{}", cresp.status());

    let req = hyper::Request::get("http://127.0.0.1:50051/".parse::<hyper::Uri>().unwrap())
        .header("Host", "127.0.0.1:50051")
        .body(Body::empty())
        .expect("create request failed");

    let resp = sender
        .send_request(req)
        .await
        .expect("send get request failed");

    let body_bytes = hyper::body::to_bytes(resp.into_body())
        .await
        .expect("body to bytes failed");

    let body = String::from_utf8(body_bytes.to_vec()).expect("msg");
    println!("{}", body);
    Ok(())
}
