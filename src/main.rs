use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Write, Result};




fn read_ssh_packet(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut lenght_buffer = [0u8; 4];
    stream.read_exact(&mut lenght_buffer)?;

    let packet_length = u32::from_be_bytes(lenght_buffer) ;

    println!("Packet lenght: {}", packet_length);

    let mut packet_data = vec![0u8; packet_length as usize];
    stream.read_exact(&mut packet_data)?;

    let padding_lenght = packet_data[0] as usize; 
    println!("Padding lenght {}", padding_lenght);

    let payload_length = packet_length as usize - padding_lenght -1;
    let payload = packet_data[1..1 + payload_length].to_vec();

    println!("payload length {}", payload.len());

    Ok(payload)

}


fn handle_client(mut stream: TcpStream) -> Result<()>{
    // SSH всегда начинает с обмена версиями в виде строки
    // Пример: "SSH-2.0-OpenSSH_9.0\r\n"

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer)?;

    let client_version_str  = String::from_utf8_lossy(&buffer[..n]);
    println!("Client version {}", client_version_str);


    if !client_version_str.starts_with("SSH-2.0-") {
        println!("Not a valid ssh client");
        return Ok(());
    }

    let server_version = "SSH-2.0-sshit_0.1\r\n"; // строка 
    stream.write_all(server_version.as_bytes())?;

    println!("Valid SSH client detected.");

    let payload = read_ssh_packet(&mut stream)?;
    println!("First message type: {}", payload[0]);

    Ok(())
}

fn build_packet(payload: Vec<u8>) -> Vec<u8> { //функция создания пакета, скорее всего идет речь о пакете, который будет проивзодить связь сервер клиент
    let block_size =8; // инициализируем первую переменную для предоставления обьема данных по обмену

    let mut padding_lenght = block_size - ((payload.len() + 5) % block_size);
    if padding_lenght < 4 {
        padding_lenght += block_size;
    }
// все что вверху абсолютно не понятно

    let packet_lenght = payload.len() + padding_lenght + 1; // тоже не понимаю для чего у нас есть длинна пакета

    let mut packet = Vec::new(); // инициализируем сам пакет

    packet.extend_from_slice(&(packet_lenght as u32).to_be_bytes()); // extend_from_slice меня пугает я не понимаю что оно делает 
    packet.push(padding_lenght as u8);
    packet.extend_from_slice(&payload);
    packet.extend(vec![0u8; padding_lenght]);


    packet // эта функция скорее всего по мои каким то наблюдениям пушит первый пакет для соединения, то есть открытый ключ после обмена которым соединение появится
}

fn write_namelist(list: &str, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&(list.len() as u32).to_be_bytes());
    buffer.extend_from_slice(list.as_bytes());
} //абсолютно не понимаю что это 

fn send_kexinit(stream: &mut TcpStream) -> std::io::Result<()> { //kexinit нихуя не понятная штука.. Что это вообще тут написано?
    let mut payload = Vec::new();

    payload.push(20);

    let cookie = [0u8; 16];// этого нет в пакете rand 
    payload.extend_from_slice(&cookie);//  эта строка и все что ниже нее один большой тупик

    write_namelist("deffie-hellman-group-sha254", &mut payload);
    write_namelist("ssh-rsa", &mut payload);
    write_namelist("aes128-ctr", &mut payload);
    write_namelist("aes128-ctr", &mut payload);
    write_namelist("hmac-sha2-256", &mut payload);
    write_namelist("hmac-sha2-256", &mut payload);
    write_namelist("none", &mut payload);
    write_namelist("none", &mut payload);
    write_namelist("", &mut payload);
    write_namelist("", &mut payload);

    payload.push(0);
    payload.extend_from_slice(&0u32.to_be_bytes());

    let packet = build_packet(payload);
    stream.write_all(&packet)?;

    println!("Sent SSH_MSG_KEXINIT");

    Ok(())
}


fn main() -> Result<()>{
    // Слушаем TCP порт (пока любой, не 22)
    let listener = TcpListener::bind("127.0.0.1:8080")?;
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
    Ok(())
}
