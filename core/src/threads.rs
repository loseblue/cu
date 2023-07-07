use tokio::runtime;
use sctp_rs::{self};
use std::io::Error;


pub fn thread_init() {

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("a init_sctp_server_thread start !");
        init_sctp_server_thread().await;
        println!("a init_sctp_client_thread start !");
        init_sctp_client_thread().await;
    });

    std::thread::sleep(std::time::Duration::from_millis(5100));
}

async fn init_sctp_server_thread() -> std::io::Result<()> {
    tokio::spawn(async {
        println!("init_sctp_server_thread start !");

        let server_address: std::net::SocketAddr = "127.0.0.1:38412".parse().unwrap();

        let server_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;
        server_socket.sctp_bindx(&[server_address], sctp_rs::BindxFlags::Add)?;
        let server_socket = server_socket.listen(100)?;
        let (accepted, _client_address) = server_socket.accept().await?;

        loop {
            let received = accepted.sctp_recv().await?;
            // match received {
            //     sctp_rs::NotificationOrData::Notification(notification)=> {
            //         // Porcess Notification
            //     },
            //     sctp_rs::NotificationOrData::Data(data) => {
            //         // Process Data
            //     }
            // }
            println!("init_sctp_server_thread end !");
        }
        Ok::<(), Error>(())
    });
    Ok::<(), Error>(())
}

async fn init_sctp_client_thread() -> std::io::Result<()> {
    println!("init_sctp_client_thread start !");

    let server_address: std::net::SocketAddr = "192.168.2.200:38412".parse().unwrap();

    let client_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;

    let (connected, assoc_id) = client_socket.sctp_connectx(&[server_address]).await?;
    println!("conected: {:#?}, assoc_id: {}", connected, assoc_id);

    for i in 0..10 {
        let message = format!("sctp-rs ping : {}", i);
        let send_data = sctp_rs::SendData {
            payload: message.as_bytes().to_vec(),
            snd_info: None,
        };
        connected.sctp_send(send_data).await?;
    }
    println!("init_sctp_client_thread end !");
    Ok::<(), Error>(())
}
