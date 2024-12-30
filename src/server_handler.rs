use std::{
    fs::File,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use once_cell::sync::OnceCell;

// Adds the necessary traits to ThreadData Structure
#[derive(Clone, Copy, Debug)]
struct ThreadData {
    stop: bool,
}

static THREAD_DATA: OnceCell<Arc<Mutex<ThreadData>>> = OnceCell::new();

impl ThreadData {
    fn handle_client(mut stream: TcpStream, thread_data: Arc<Mutex<ThreadData>>) {
        let file = File::open("bootstrap.css").expect("Unable to open file");
        let file_size = file.metadata().unwrap().len();

        let mut reader = BufReader::new(file);
        let mut buffer = [0; 512];

        // Send file size as the initial message
        let _ = stream.write_all(format!("FILE_SIZE={file_size}--FILE_SIZE").as_bytes());

        loop {
            let bytes_read = reader.read(&mut buffer).expect("Error reading file");

            // Write buffer to client
            let _ = stream
                .write_all(&buffer[..bytes_read])
                .map_err(|err| eprintln!("{}", err));

            // Check for "TRANSFER_COMPLETE" message
            let thread_lock = thread_data.lock().unwrap();

            if String::from_utf8_lossy(&buffer).contains("TRANSFER_COMPLETE") {
                Self::set_stop(&mut thread_data.lock().unwrap(), true);
            }

            if thread_lock.get_stop() {
                println!("TRANSFER_COMPLETE RECEIVED");
                break;
            }

            // println!("{}", thread_lock.get_stop())
        }
    }

    fn setup() -> ThreadData {
        Self { stop: false }
    }

    // Getter
    fn get_stop(&self) -> bool {
        self.stop
    }

    // Accessor
    fn set_stop(&mut self, stop_value: bool) {
        self.stop = stop_value;
    }
}

pub fn setup() {
    THREAD_DATA
        .set(Arc::new(Mutex::new(ThreadData::setup())))
        .unwrap();
}

fn client() {
    let mut data: Vec<u8> = Vec::new();
    let mut file = File::create("output.css").expect("Couldn't make file");

    let mut final_file_size = 0;
    let mut bytes_received = 0;

    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Connection Successful");

            let mut buffer = [0; 512];

            loop {
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        bytes_received += bytes_read;

                        let received_data = &buffer[..bytes_read];
                        let string_data = String::from_utf8_lossy(received_data);

                        if string_data.contains("FILE_SIZE=") {
                            // Extracts the file size from the first chunk sent
                            final_file_size = string_data[string_data.find("=").unwrap() + 1
                                ..string_data.find("--FILE_SIZE").unwrap()]
                                .parse()
                                .unwrap();

                            // Writes to the file without any of the header info
                            data.extend_from_slice(
                                &buffer[format!("FILE_SIZE={}--FILE_SIZE", final_file_size).len()
                                    ..received_data.len()],
                            );
                            continue;
                        }

                        println!("Received {} bytes", bytes_read);

                        // Append the valid portion of the buffer to `data`
                        data.extend_from_slice(&buffer[..bytes_read]);

                        if bytes_received >= final_file_size {
                            println!("Transfer Complete");
                            let _ = stream
                                .write(b"TRANSFER_COMPLETE")
                                .map_err(|err| println!("{}", err));
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Failed to read response: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    // Write the collected data to the file
    if let Err(err) = file.write_all(&data) {
        eprintln!("Failed to write to file: {}", err);
    }
}

pub fn server() -> Result<String, String> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Unable to bind 8080");

    println!("Listening on 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let thread_data = THREAD_DATA.get().unwrap();

                println!("New connection: {:?}", stream.peer_addr().unwrap());
                let _ = thread::Builder::new()
                    .name("tcp_thread".to_string())
                    .spawn(move || ThreadData::handle_client(stream, thread_data.clone()));
            }
            Err(e) => {
                return Err(format!("Connection Failed: {}", e));
            }
        }
    }
    return Ok("Connection Successful".to_string());
}

