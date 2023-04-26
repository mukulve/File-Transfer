use local_ip_address::local_ip;
use reqwest;
use std::{
    fs,
    fs::File,
    io::{stdin, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let mut user_choice = String::new();

    println!("⌨️ Enter '1' To Share File OR '2' To Receive File : ⌨️");
    stdin().read_line(&mut user_choice).unwrap();
    let user_choice = user_choice.trim();

    if user_choice == "1" {
        match send() {
            Ok(_) => println!("✅ Sent Data ✅"),
            Err(e) => println!("⚠️ Could Not Send Data ⚠️ {}", e),
        }
    } else if user_choice == "2" {
        match receive() {
            Ok(_) => println!("✅ Got Data ✅"),
            Err(e) => println!("⚠️ Could Not Receive Data ⚠️ {}", e),
        }
    } else {
        println!("⚠️ Wrong Input! ⚠️");
    }
}

fn send() -> Result<(), Box<dyn std::error::Error>> {
    let my_local_ip = local_ip()?;

    println!("👂 Listening On Port 7878 👂");
    println!("🌐 Request URL = '{}:7878' 🌐", my_local_ip);

    let listener = TcpListener::bind(format!("{}:7878", my_local_ip))?;

    let mut file_name = String::new();
    println!("⌨️ Enter File To Share : ⌨️");
    stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim();
    println!("⌛ Waiting For Connection ⌛");
    for stream in listener.incoming().take(1) {
        let stream: TcpStream = stream?;
        handle_connection(stream, file_name.to_string());
    }
    return Ok(());
}

fn receive() -> Result<(), Box<dyn std::error::Error>> {
    let mut request_url = String::new();
    println!("⌨️ Enter Request URL : ⌨️");
    stdin().read_line(&mut request_url).unwrap();
    let request_url = request_url.trim();

    let resp = reqwest::blocking::get(&format!("http://{}", request_url))?.text()?;

    println!("{}", resp);
    write_file(resp);

    Ok(())
}

fn handle_connection(mut stream: TcpStream, file_name: String) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn write_file(file_content: String) {
    let mut file = File::create("receivedFile.txt").unwrap();
    writeln!(&mut file, "{}", file_content.trim_end()).unwrap();
}
