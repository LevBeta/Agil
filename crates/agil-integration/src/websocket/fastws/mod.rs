use crate::net::url::Url;
use fastwebsockets::{FragmentCollector, Frame};
use hyper::upgrade::Upgraded;
use hyper::{
    header::{CONNECTION, UPGRADE},
    Request,
};
use hyper_util::rt::TokioIo;
use std::{future::Future, sync::Arc};
use tokio::net::TcpStream;
use tokio_rustls::{
    rustls::{ClientConfig, RootCertStore},
    TlsConnector,
};
pub mod message;

pub type FastWebSocket = FragmentCollector<TokioIo<Upgraded>>;

pub type FastWsMessage<'a> = Frame<'a>;

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}

pub async fn connect(url: Url) -> FragmentCollector<TokioIo<Upgraded>> {
    let tcp_stream = TcpStream::connect(&url.get_domain_and_port())
        .await
        .unwrap();
    let tls_connector = tls_connector();
    let domain = tokio_rustls::rustls::pki_types::ServerName::try_from(url.get_domain()).unwrap();
    let tls_stream = tls_connector.connect(domain, tcp_stream).await.unwrap();

    let req = Request::builder()
        .method("GET")
        .uri(&url.get_full())
        .header("Host", &url.get_domain_and_port())
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header(
            "Sec-WebSocket-Key",
            fastwebsockets::handshake::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(http_body_util::Empty::<bytes::Bytes>::new())
        .unwrap();

    let (ws, _) = fastwebsockets::handshake::client(&SpawnExecutor, req, tls_stream)
        .await
        .unwrap();

    FragmentCollector::new(ws)
}

pub fn tls_connector() -> TlsConnector {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    TlsConnector::from(Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net::url::{Protocol, Url};
    use crate::websocket::fastws::message::FastWebSocketParser;
    use crate::websocket::message::StreamParser;
    use fastwebsockets::{Frame, Payload};
    #[tokio::test]
    async fn connect_test() {
        let u = Url {
            domain: "stream.bybit.com".into(),
            path: "/v5/public/spot".into(),
            port: Some(443),
            protocol: Some(Protocol::WSS),
        };

        let mut ws = connect(u).await;
        let message = r#"
        {
            "op": "subscribe",
            "args": [
                "publicTrade.BTCUSDT"
            ]
        }"#;
        let mut binding = message.as_bytes().to_vec();
        let frame = Frame::text(Payload::BorrowedMut(&mut binding));
        ws.write_frame(frame).await.unwrap();
        loop {
            let msg = ws.read_frame().await;

            let x = FastWebSocketParser::parse::<String>(msg);
        }
    }
}
