use tokio::runtime;
use sctp_rs::{self};


pub fn thread_init() {
    // let core_ids = core_affinity::get_core_ids().unwrap();

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let core_ids_map = [2, 5, 6, 8];

    rt.block_on(async {
        init_sctp_server_thread().await;
        init_sctp_client_thread().await;
        
        // for idx in &core_ids_map {
            
            // println!("core_id = {}", idx);
            // let core_id = core_ids[*idx];

            // tokio::spawn(async move {
                // let res = core_affinity::set_for_current(core_id);
                // if res {
                    // init_sctp_server_thread().await;
                    // init_sctp_client_thread().await;
 
                // }

            // });
        // }
    });
}

async fn init_sctp_server_thread() -> std::io::Result<()> {
    println!("init_sctp_server_thread start !");

    let server_address: std::net::SocketAddr = "127.0.0.1:38472".parse().unwrap();

    let server_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;
    server_socket.sctp_bindx(&[server_address], sctp_rs::BindxFlags::Add)?;

    let server_socket = server_socket.listen(10)?;

    let (accepted, _client_address) = server_socket.accept().await?;


    let received = accepted.sctp_recv().await?;
    match received {
        sctp_rs::NotificationOrData::Notification(notification)=> {
            // Porcess Notification
        },
        sctp_rs::NotificationOrData::Data(data) => {
            // Process Data
        }
    }
    println!("init_sctp_server_thread end !");

    Ok(())
}

async fn init_sctp_client_thread() -> std::io::Result<()> {
    println!("init_sctp_client_thread start !");

    let server_address: std::net::SocketAddr = "192.168.2.200:38472".parse().unwrap();

    let client_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;

    let (connected, assoc_id) = client_socket.sctp_connectx(&[server_address]).await?;
    eprintln!("conected: {:#?}, assoc_id: {}", connected, assoc_id);

    for i in 0..10 {
        let message = format!("sctp-rs ping : {}", i);
        let send_data = sctp_rs::SendData {
            payload: message.as_bytes().to_vec(),
            snd_info: None,
        };
        connected.sctp_send(send_data).await?;
        let received = connected.sctp_recv().await?;
        eprintln!("received: {:#?}", received);
    }
    println!("init_sctp_client_thread end !");

    Ok(())
}
