use crate::tcp::TcpState;
use crate::udp::UdpState;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct State {
    pub download_root: Option<PathBuf>,
    pub tcp_sockets: HashMap<u32, TcpState>,
    pub udp_sockets: HashMap<u32, UdpState>,
    pub next_socket_id: u32,
}

impl State {
    pub fn new() -> Self {
        Self {
            download_root: None,
            tcp_sockets: HashMap::new(),
            udp_sockets: HashMap::new(),
            next_socket_id: 1,
        }
    }

    pub fn next_id(&mut self) -> u32 {
        let id = self.next_socket_id;
        self.next_socket_id = self.next_socket_id.wrapping_add(1);
        id
    }
}
