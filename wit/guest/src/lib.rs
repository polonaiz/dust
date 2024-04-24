use std::collections::HashMap;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use wasi::sockets::network::ErrorCode;
use wasi::sockets::network::{IpAddressFamily, IpSocketAddress, Ipv4SocketAddress};
use wasi::sockets::tcp::{InputStream, OutputStream, TcpSocket};
use wasi::sockets::tcp_create_socket::create_tcp_socket;
use wasi::sockets::udp::UdpSocket;
use wasi::sockets::udp_create_socket::create_udp_socket;

use crate::wasi::io::streams::StreamError;
use crate::wasi::sockets::udp::OutgoingDatagram;

wit_bindgen::generate!({
    // world: "dust:kernel/particle",
    path: "../wit",
});

lazy_static::lazy_static! {
    static ref ASYNC_RUNTIME: Runtime = Builder::new_current_thread().build().unwrap();
    static ref TCP_LISTEN_SOCKET: TcpSocket = create_tcp_socket(IpAddressFamily::Ipv4).unwrap();
    static ref TCP_ACCEPT_SOCKET_HASHMAP: std::sync::Mutex<HashMap<u32,(TcpSocket, InputStream, OutputStream)>> =
        std::sync::Mutex::new(HashMap::new());
    static ref UDP_BIND_SOCKET: UdpSocket = create_udp_socket(IpAddressFamily::Ipv4).unwrap();
}

struct Particle {}

impl Guest for Particle {
    fn bootstrap(_config: String) -> String {
        ASYNC_RUNTIME.block_on(async move { println!("async in bootstrap") });

        let instance_network = wasi::sockets::instance_network::instance_network();
        let listen_port = 17321;

        //
        TCP_LISTEN_SOCKET
            .start_bind(
                &instance_network,
                IpSocketAddress::Ipv4(Ipv4SocketAddress {
                    port: listen_port,
                    address: (0, 0, 0, 0),
                }),
            )
            .unwrap();
        TCP_LISTEN_SOCKET.finish_bind().unwrap();
        TCP_LISTEN_SOCKET.start_listen().unwrap();
        TCP_LISTEN_SOCKET.finish_listen().unwrap();

        //
        UDP_BIND_SOCKET
            .start_bind(
                &instance_network,
                IpSocketAddress::Ipv4(Ipv4SocketAddress {
                    port: listen_port,
                    address: (0, 0, 0, 0),
                }),
            )
            .unwrap();
        UDP_BIND_SOCKET.finish_bind().unwrap();

        "from_boot".to_string()
    }

    fn poll(_input: String) -> String {
        ASYNC_RUNTIME.block_on(async move { println!("async in poll") });

        //
        let result = TCP_LISTEN_SOCKET.accept();
        match result {
            Err(ErrorCode::WouldBlock) => (),
            Err(e) => {
                panic!("{}", e);
            }
            Ok(acceptance) => {
                let accept_socket_handle = acceptance.0.handle();
                TCP_ACCEPT_SOCKET_HASHMAP
                    .lock()
                    .unwrap()
                    .insert(accept_socket_handle, acceptance);
                println!(
                    "ACCEPT_SOCKET_VECTOR.len: {:?}",
                    TCP_ACCEPT_SOCKET_HASHMAP.lock().unwrap().len()
                );
            }
        }

        //
        let mut closed_socket_handles = Vec::<u32>::new();
        for (handle, acceptance) in TCP_ACCEPT_SOCKET_HASHMAP.lock().unwrap().iter() {
            let read_result = acceptance.1.read(4096);
            if let Err(StreamError::Closed) = read_result {
                let closed_socket_handle = *handle;
                closed_socket_handles.push(closed_socket_handle);
                println!("closed_socket_handle queued: {:?}", closed_socket_handle);
            } else if let Err(StreamError::LastOperationFailed(_)) = read_result {
                panic!("{:?}", read_result);
            } else {
                let read_bytes = read_result.unwrap();
                println!("read_bytes: {:?}", String::from_utf8(read_bytes.clone()));

                let output_steram = &acceptance.2;
                output_steram.write(&read_bytes).unwrap();
            }
        }
        for closed_socket_handle in closed_socket_handles {
            println!(
                "closed_socket_handle remove: begin: {:?}",
                closed_socket_handle
            );
            let acceptance = TCP_ACCEPT_SOCKET_HASHMAP
                .lock()
                .unwrap()
                .remove(&closed_socket_handle)
                .unwrap();
            println!(
                "closed_socket_handle remove: end: {:?}",
                closed_socket_handle
            );
            println!(
                "ACCEPT_SOCKET_HASHMAP.len: {:?}",
                TCP_ACCEPT_SOCKET_HASHMAP.lock().unwrap().len()
            );

            println!("drop tcp_socket: begin");
            let (tcp_socket, input_stream, output_stream) = acceptance;
            drop(input_stream);
            drop(output_stream);
            drop(tcp_socket);
            println!("drop tcp_socket: end");
        }

        //
        let (incoming_datagram_stream, outgoing_datagram_stream) =
            UDP_BIND_SOCKET.stream(Option::None).unwrap();
        let incoming_datagrams = incoming_datagram_stream.receive(4096).unwrap();
        let _: Vec<()> = incoming_datagrams
            .iter()
            .map(|incoming_datagram| println!("incoming_datagram: {:?}", incoming_datagram))
            .collect();
        let outgoing_datagrams: Vec<OutgoingDatagram> = incoming_datagrams
            .iter()
            .map(|incoming_datagram| OutgoingDatagram {
                data: incoming_datagram.data.clone(),
                remote_address: Option::Some(incoming_datagram.remote_address.clone()),
            })
            .collect();
        let mut total_sent_datagram_count = 0;
        while total_sent_datagram_count < outgoing_datagrams.len() {
            let sendable_datagram_count = outgoing_datagram_stream.check_send().unwrap();
            let sending_datagram_count = std::cmp::min(
                sendable_datagram_count as usize,
                outgoing_datagrams.len() - total_sent_datagram_count,
            );
            let sent_datagram_count = outgoing_datagram_stream
                .send(
                    &outgoing_datagrams[total_sent_datagram_count
                        ..total_sent_datagram_count + sending_datagram_count],
                )
                .unwrap();
            total_sent_datagram_count += sent_datagram_count as usize;
        }

        // let sent_diagram = outgoing_datagram_stream.send(&outgoing_datagrams).unwrap();
        // if sent_diagram != outgoing_datagrams.len() as u64 {
        //     panic!(
        //         "sent_diagram != outgoing_datagrams.len(): {}, {}",
        //         sent_diagram,
        //         outgoing_datagrams.len()
        //     );
        // }

        "from_tick".to_string()
    }

    fn cleanup(_input: String) -> String {
        ASYNC_RUNTIME.block_on(async move { println!("async in cleanup") });

        "from_shutdown".to_string()
    }
}

export!(Particle);
