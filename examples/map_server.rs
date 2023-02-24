use keyboarder::{platform_impl::Simulator, simulate::Simulate, types::KeyEventBin};
use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request = Vec::new();
    let _size = buf_reader.read_to_end(&mut http_request).unwrap();

    let key_event_bin = KeyEventBin::new(http_request);
    let key_event = key_event_bin.to_key_event().unwrap();
    log::info!("simulate event: {:?}", &key_event);

    Simulator::event_to_server(&key_event)
        .map_err(|err| log::error!("simulate err: {:?}", err))
        .ok();
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let _handle = Simulator::spawn_server()?;

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
