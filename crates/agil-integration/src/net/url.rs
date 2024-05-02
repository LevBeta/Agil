use std::borrow::Cow;

/// A simple [`Url`] struct to simplify url passing to various types of processes
///
/// This should have a complete refactor, it's really poor code
/// made to speed up the testing of some parts of agil-data
pub struct Url {
    pub domain: Cow<'static, str>,
    pub path: Cow<'static, str>,
    pub port: Option<u16>,
    pub protocol: Option<Protocol>,
}

impl Url {
    /// Construct a new 'Url' instance
    pub fn new<S>(domain: S, path: S, port: Option<u16>, protocol: Option<Protocol>) -> Url
    where
        S: Into<Cow<'static, str>>,
    {
        Url {
            domain: domain.into(),
            path: path.into(),
            port,
            protocol,
        }
    }

    /// Returns the full URL as a string.
    /// Protocol - Domain - Port - Path
    pub fn get_full(&self) -> String {
        let port_string = match self.port {
            Some(p) => format!(":{}", p),
            None => self
                .protocol
                .as_ref()
                .and_then(|p| p.default_port())
                .map_or_else(|| "".to_string(), |p| format!(":{}", p)),
        };

        format!(
            "{}{}{}{}",
            self.protocol.as_ref().map_or("", |p| p.scheme()),
            self.domain,
            port_string,
            self.path
        )
    }

    /// Returns the domain and port
    /// Domain - Port
    pub fn get_domain_and_port(&self) -> String {
        let port_string = match self.port {
            Some(p) => format!(":{}", p),
            None => self
                .protocol
                .as_ref()
                .and_then(|p| Protocol::default_port(p))
                .map_or_else(|| "".to_string(), |p| format!(":{}", p)),
        };

        format!("{}{}", self.domain, port_string)
    }
    /// Returns only the domain
    /// Domain
    pub fn get_domain(&self) -> String {
        String::from(self.domain.as_ref())
    }
}

/// Enum representing different url protocols.
#[derive(Clone)]
pub enum Protocol {
    HTTP,
    HTTPS,
    WS,
    WSS,
}

impl Protocol {
    /// Returns the scheme associated with the procol
    pub fn scheme(&self) -> &'static str {
        match self {
            Self::HTTP => "http://",
            Self::HTTPS => "https://",
            Self::WS => "ws://",
            Self::WSS => "wss://",
        }
    }

    /// Returns the default port associated with the protocol
    pub fn default_port(&self) -> Option<u16> {
        match self {
            Self::HTTP => Some(80),
            Self::HTTPS => Some(443),
            Self::WS => Some(80),
            Self::WSS => Some(443),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_with_protocol() {
        let url = Url::new(
            "stream.bybit.com",
            "/v5/public/linear",
            None,
            Some(Protocol::WSS),
        );

        let s = url.get_full();
        assert_eq!(s, "wss://stream.bybit.com:443/v5/public/linear");
    }

    #[test]
    fn test_get_domain_and_port() {
        let url = Url::new("example.com", "/path", Some(8080), Some(Protocol::HTTP));

        let s = url.get_domain_and_port();
        assert_eq!(s, "example.com:8080");
    }

    #[test]
    fn test_get_full_without_protocol() {
        let url = Url::new("example.com", "/path", Some(8080), None);

        let s = url.get_full();
        assert_eq!(s, "example.com:8080/path");
    }
}
