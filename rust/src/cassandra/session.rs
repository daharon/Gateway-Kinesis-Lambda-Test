use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{NodeTcpConfigBuilder, ClusterTcpConfig, TcpConnectionPool};
use cdrs::cluster::session::Session;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;


pub static CASSANDRA_KEYSPACE: &str = "test_ks";
pub static CASSANDRA_TABLE: &str    = "test_table";
pub type CassandraSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;


pub fn new<'a>(host: &str, port: i32) -> CassandraSession {
    let addr = format!("{}:{}", host, port);
    let node = NodeTcpConfigBuilder::new(&addr, NoneAuthenticator {})
        .build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let session = cdrs::cluster::session::new(
        &cluster_config,
        cdrs::load_balancing::RoundRobin::new())
        .expect("Failed to establish Cassandra session");

    create_cassandra_keyspace(&session);
    create_cassandra_table(&session);
    session
}

fn create_cassandra_keyspace(session: &CassandraSession) {
    let create_ks = format!(r#"
            CREATE KEYSPACE IF NOT EXISTS {}
            WITH REPLICATION = {{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 }};
        "#, CASSANDRA_KEYSPACE);
    session.query(&create_ks)
        .expect("Keyspace creation error");
}

fn create_cassandra_table(session: &CassandraSession) {
    let create_table = format!(r#"
            CREATE TABLE IF NOT EXISTS {}.{} (
                id int PRIMARY KEY,
                description varchar,
                count int
            );"#, CASSANDRA_KEYSPACE, CASSANDRA_TABLE);
    session.query(&create_table)
        .expect("Table creation error");
}
