#[cfg(test)]
mod tests {
    use crate::rpc;
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn test_rpc() {
        let (tx, rx) = mpsc::channel::<()>();

        let rpc_handle = thread::spawn(move || {
            let rpc_srv = rpc::RPC::new("127.0.0.1:3456", rx);
            rpc_srv.spin();
        });

        let rpc_done_handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            let _ = tx.send(());
            println!("send done");
        });

        rpc_done_handle.join().unwrap();
        rpc_handle.join().unwrap();
    }
}
