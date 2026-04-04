use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
use crate::threadpool::ThreadPool;
use crate::client_handler::Client;
use rand;

fn broadcast_message(clients: Arc<Mutex<HashMap<usize, TcpStream>>>,
                     msg: String, client: Arc<Mutex<Client>>) {

    let client_id: usize = {
        let temp_client: MutexGuard<Client> = client.lock().unwrap();
        *temp_client.ref_server_id()
    };
    let mut temp_clients: MutexGuard<HashMap<usize, TcpStream>> =  clients.lock().unwrap();

    for (id, stream_writer) in temp_clients.iter_mut() {

        if *id != client_id {

            if let Err(e) = stream_writer.write_all(msg.as_bytes()) {
                println!("The error in broadcasting message: {}", e);
                break;
            }

            stream_writer.flush().unwrap();
        }

    }

}

fn get_from_client(mut stream_reader: TcpStream, client: Arc<Mutex<Client>>,
                   clients: Arc<Mutex<HashMap<usize, TcpStream>>>) {
    let mut buffer: [u8; 2048] = [0; 2048];

    loop {

        match stream_reader.read(&mut buffer) {
            Ok(0) => {

                {
                    let mut temp_clients: MutexGuard<HashMap<usize, TcpStream>> = clients.lock().unwrap();

                    temp_clients.remove(&{
                        let temp_client: MutexGuard<Client> = client.lock().unwrap();
                        *temp_client.ref_server_id()
                    });
                }

                let new_msg: String;

                {
                    let temp_client: MutexGuard<Client> = client.lock().unwrap();
                    new_msg = format!("SERVER: The User with username:: {} has left the group chat", temp_client.ref_username().unwrap());
                }

                broadcast_message(Arc::clone(&clients), new_msg, Arc::clone(&client));

                break;
            }
            Ok(n) => {
                let msg: String = String::from_utf8_lossy(&buffer[0..n]).to_string();
                let new_msg: String;

                if msg.starts_with("Username: ") {
                    let username: String = msg[10..].to_string();
                    new_msg = format!("SERVER: User with username:: {} Joined the group chat", username);

                    {
                        let mut temp_client: MutexGuard<Client> = client.lock().unwrap();
                        temp_client.set_username(username);
                    }

                } else {
                    let username: String = {
                        let temp_client: MutexGuard<Client> = client.lock().unwrap();
                        temp_client.ref_username().unwrap().clone()
                    };

                    new_msg = format!("SERVER: {}:: {}", username, msg);
                }

                broadcast_message(Arc::clone(&clients), new_msg, Arc::clone(&client));
            }
            Err(r) => {
                println!("2.Err: {}", r);
                break;
            }
        }

    }

}

pub fn server() {
    let pool: ThreadPool = ThreadPool::new(3);
    let listener: TcpListener = TcpListener::bind("127.0.0.7:8080").unwrap();
    let permanent_clients: Arc<Mutex<HashMap<usize, TcpStream>>> = Arc::new(Mutex::new(HashMap::with_capacity(3)));
    let mut thread_id: usize;

    println!("Listening to port 8080....");

    for res_stream in listener.incoming() {
        thread_id = rand::random_range(0..100);
        let clients: Arc<Mutex<HashMap<usize, TcpStream>>> = Arc::clone(&permanent_clients);
        let client: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new()));

        match res_stream {
            Ok(stream) => {

                {
                    let mut temp_clients: MutexGuard<HashMap<usize, TcpStream>> = clients.lock().unwrap();
                    let stream_writer: TcpStream = stream.try_clone().unwrap();

                    while temp_clients.contains_key(&thread_id) {
                        thread_id = rand::random_range(0..100);
                    }

                    temp_clients.insert(thread_id, stream_writer);
                }

                {
                    let mut temp_client: MutexGuard<Client> = client.lock().unwrap();
                    temp_client.set_id(thread_id);
                }

                pool.execute(move || {
                    get_from_client(stream, client, clients)
                });
            }
            Err(e) => {
                println!("Err: {}", e);
                break;
            }
        }
    }
}