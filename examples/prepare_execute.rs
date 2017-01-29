extern crate cdrs;

use cdrs::client::CDRS;
use cdrs::query::QueryParamsBuilder;
use cdrs::authenticators::PasswordAuthenticator;
use cdrs::compression::Compression;
use cdrs::consistency::Consistency;
use cdrs::transport::Transport;

// default credentials
const USER: &'static str = "cassandra";
const PASS: &'static str = "cassandra";
const ADDR: &'static str = "127.0.0.1:9042";

fn main() {
    let authenticator = PasswordAuthenticator::new(USER, PASS);
    let tcp_transport = Transport::new(ADDR).unwrap();
    let client = CDRS::new(tcp_transport, authenticator);
    let mut session = client.start(Compression::None).unwrap();

    // NOTE: keyspace "keyspace" should already exist
    let create_table_cql = "USE keyspace;".to_string();
    let with_tracing = false;
    let with_warnings = false;

    let prepared = session.prepare(create_table_cql, with_tracing, with_warnings)
        .unwrap()
        .get_body()
        .into_prepared()
        .unwrap();

    println!("prepared:\n{:?}", prepared);

    let execution_params = QueryParamsBuilder::new(Consistency::One).finalize();
    let query_id = prepared.id;
    let executed = session.execute(query_id, execution_params, false, false)
        .unwrap()
        .get_body()
        .into_set_keyspace()
        .unwrap();

    println!("executed:\n{:?}", executed);
}
