use tokio::sync::mpsc;

mod tcp_connection;

pub async fn run_server(mut reciever: mpsc::Receiver<String>) {
    let mut connection = tcp_connection::client::new("127.0.0.1".to_string(), 3333);

    while let Some(chesss_move) = reciever.recv().await {
        connection.send(chesss_move.as_bytes());
        println!("message sent succesfuly");
    }
}
