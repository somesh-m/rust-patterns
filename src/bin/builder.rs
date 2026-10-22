/*Builder pattern is used to build complex object incrementally, particularly used when there are lot of configuration parameters
 */

#[derive(Debug)]
struct TcpServer {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_ms: u64,
    tls_enabled: bool,
}

struct TcpServerBuilder {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_ms: u64,
    tls_enabled: bool,
}

impl TcpServerBuilder {
    fn new() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
            max_connections: 100,
            timeout_ms: 3000,
            tls_enabled: false,
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.into();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn max_connections(mut self, count: usize) -> Self {
        self.max_connections = count;
        self
    }

    fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = timeout;
        self
    }

    fn tls_enabled(mut self, enabled: bool) -> Self {
        self.tls_enabled = enabled;
        self
    }

    fn build(self) -> Result<TcpServer, String> {
        if self.port == 0 {
            return Err("Port cannot be zero".into());
        }

        if self.max_connections == 0 {
            return Err("Max connection must be positive".into());
        }

        Ok(TcpServer {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout_ms: self.timeout_ms,
            tls_enabled: self.tls_enabled,
        })
    }
}

fn main() {
    let server = TcpServerBuilder::new()
        .host("0.0.0.0")
        .port(9000)
        .max_connections(5000)
        .timeout_ms(10000)
        .tls_enabled(true)
        .build()
        .unwrap();

    println!("{server:#?}");
}
