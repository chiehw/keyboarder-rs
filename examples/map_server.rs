use keyboarder::{
    platform_impl::Simulator,
    simulate::Simulate,
    types::{ServerMode, SimEvent},
};
use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
    process::exit,
};

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request = Vec::new();
    let _size = buf_reader.read_to_end(&mut http_request)?;

    let sim_event = SimEvent::try_from(http_request)?;
    Simulator::event_to_server(&sim_event)
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("DISPLAY", ":0");

    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let _handle = Simulator::spawn_server(ServerMode::Translate)?;

    ctrlc::set_handler(move || {
        Simulator::event_to_server(&SimEvent::ExitThread)
            .map_err(|err| log::error!("Failed to exit thread: {:?}", err))
            .ok();
        exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream: TcpStream = stream;
                if let Err(err) = handle_connection(stream) {
                    Simulator::event_to_server(&SimEvent::ReleaseKeys)
                        .map_err(|err| log::error!("Failed to exit thread: {:?}", err))
                        .ok();
                    log::error!("simulate err: {:?}", err);
                }
            }

            Err(err) => {
                log::error!("Faile to process stream: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
