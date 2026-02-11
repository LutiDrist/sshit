use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};




fn handle_client(mut stream: TcpStream) {
    // SSH всегда начинает с обмена версиями в виде строки
    // Пример: "SSH-2.0-OpenSSH_9.0\r\n"

    let server_version = b"SSH-2.0-sshit_0.1\r\n"; // строка 

    if let Err(e) = stream.write_all(server_version) {
        eprintln!("Failed to send server version {}", e);
        return;
    }
    // Буфер для чтения версии клиента
    let mut buffer = [0u8; 512];

    // Читаем то, что пришло от клиента
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read client version {}",e );
            return;
        }
    };

    let client_version = &buffer[..bytes_read];
    
    let client_version_str = String::from_utf8_lossy(client_version);

    println!(
        "Client version: {}",
        client_version_str
    );

    if !client_version_str.starts_with("SSH-2.0-") {
        eprintln!("Not an SSH client. Closing connection");
        return;
    }


    println!("Valid SSH client detected. Proceeding to next stage...");
}

fn main() {
    // Слушаем TCP порт (пока любой, не 22)
    let listener = TcpListener::bind("127.0.0.1:8088")
        .expect("Failed to bind");

    println!("sshit listening on 127.0.0.1:8088");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
