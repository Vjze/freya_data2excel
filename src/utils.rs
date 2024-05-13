use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub async fn band_data_sql_client() -> Client<Compat<TcpStream>> {
    let mut config = config();
    config.database("Band_Data");
    let tcp = match TcpStream::connect(config.get_addr()).await{
        Ok(t) => t,
        Err(_) => {
            config.host("192.168.100.250");
            TcpStream::connect(config.get_addr()).await.unwrap()
        },
    };
    tcp.set_nodelay(true).unwrap();
    let client = tiberius::Client::connect(config, tcp.compat_write())
        .await
        .unwrap();
    client
}
fn config() -> Config {
    let mut config = Config::new();
    // config.host("192.168.0.230");
    // config.authentication(AuthMethod::sql_server("tester", "test1234"));
    // config.host("192.168.3.120");
    // config.authentication(AuthMethod::sql_server("sa", "a1"));
    // config.host("192.168.3.250");
    config.host("192.168.3.250");
    config.authentication(AuthMethod::sql_server("osatest", "osatest"));
    config.port(1433);
    config.trust_cert();
    config
}