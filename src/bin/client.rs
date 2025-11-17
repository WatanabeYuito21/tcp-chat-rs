use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    println!("サーバーに接続中...");

    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => {
            println!("サーバーに接続しました!");
            stream
        }
        Err(e) => {
            eprintln!("接続失敗: {}", e);
            return;
        }
    };

    println!("メッセージを入力してください('quit'で終了):");

    let stdin = io::stdin();
    let mut buffer = [0; 1024];

    loop {
        // ユーザーからの入力を取得
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut input).expect("入力の読み取りに失敗");

        let input = input.trim();

        if input == "quit" {
            println!("接続を終了します。");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // サーバーに送信
        if let Err(e) = stream.write_all(input.as_bytes()) {
            eprintln!("送信エラー: {}", e);
            break;
        }

        // サーバーからのレスポンスを受信
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("サーバーが切断しました。");
                    break;
                }
                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("サーバーからの応答: {}", response);
            }
            Err(e) => {
                eprintln!("受信エラー: {}", e);
                break;
            }
        }
    }
}
