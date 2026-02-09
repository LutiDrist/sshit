use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    // SSH всегда начинает с обмена версиями в виде строки
    // Пример: "SSH-2.0-OpenSSH_9.0\r\n"

    let server_version = b"SSH-2.0-sshit_0.1\r\n";

    // Отправляем нашу версию клиенту
    stream.write_all(server_version).unwrap();

    // Буфер для чтения версии клиента
    let mut buffer = [0u8; 256];

    // Читаем то, что пришло от клиента
    let bytes_read = stream.read(&mut buffer).unwrap();

    let client_version = &buffer[..bytes_read];

    println!(
        "Client version: {}",
        String::from_utf8_lossy(client_version)
    );
}

fn main() {
    // Слушаем TCP порт (пока любой, не 22)
    let listener = TcpListener::bind("127.0.0.1:2222")
        .expect("Failed to bind");

    println!("sshit listening on 127.0.0.1:2222");

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
