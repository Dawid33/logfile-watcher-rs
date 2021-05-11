use {
    std::sync::mpsc::Sender,
    tungstenite::Message,
    url::Url,
};

/// Creates a new connection to the specified url in a new thread.
/// If the thread exits an error is sent over the mspc channel.
pub fn connect(url: Url, rx: Sender<tungstenite::error::Error>) {
    std::thread::spawn(move || {
        let stream = std::net::TcpStream::connect(&*url.socket_addrs(|| None).unwrap());
        let stream = stream.unwrap();
        let (mut socket, _response) = tungstenite::client(url, stream).unwrap();

        if let Err(e) = socket.write_message(Message::Text("Hello WebSocket".into())) {
            rx.send(e).unwrap();
            return ();
        }

        loop {
            match socket.read_message() {
                Ok(msg) => println!("Received: {}", msg),
                Err(e) => {
                    rx.send(e).unwrap();
                    return ();
                }
            }
        }
    });
}
