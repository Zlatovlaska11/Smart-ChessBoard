use tokio::sync::{mpsc, oneshot};

mod tcp_connection;

pub async fn run_server(mut reciever: mpsc::Receiver<String>, ready_tx: oneshot::Sender<String>) {
    let mut connection = tcp_connection::client::new("127.0.0.1".to_string(), 3333).await;

    let position = connection.GameStart().await;

    println!("possition =>{}", position);
    let _ = ready_tx.send(position);

    while let Some(chesss_move) = reciever.recv().await {
        let _ = connection.send(chesss_move.as_bytes()).await;

        println!("message sent succesfuly");
        if chesss_move == "END\n" {
            println!("sent end of game signal");
            return;
        }
    }
}
