use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!(
        "新しいClientが接続しました: {}",
        stream.peer_addr().unwrap()
    );

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // クライアントが切断した場合
                println!("Clientが切断しました: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                // 受信したデータを表示
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("受信: {}", received.trim());

                // エコー応答を送信
                let response = format!("サーバーが受信しました。: {}", received);
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("送信エラー: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("読み取りエラー: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("ポートのバインドに失敗");
    println!("サーバーが起動しました。ポート8080で待機中...");
    println!("接続を待っています...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 各クライアントを別スレッドで処理
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("接続エラー: {}", e);
            }
        }
    }
}
