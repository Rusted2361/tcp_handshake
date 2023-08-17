
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

// Ethereum protocol magic bytes
const ETHEREUM_MAGIC: [u8; 4] = [0x22, 0x80, 0x9D, 0xF2];
// Ethereum protocol version
const ETHEREUM_VERSION: u32 = 5;

//handshake function 
fn perform_ethereum_handshake<A: ToSocketAddrs>(addr: A) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr)?;

    // Send the Ethereum handshake
    let mut handshake = Vec::new();
    handshake.extend_from_slice(&ETHEREUM_MAGIC);
    handshake.extend_from_slice(&ETHEREUM_VERSION.to_be_bytes());

    // Send the handshake payload
    stream.write_all(&handshake)?;

    // Read and process the response
    let mut response_magic = [0u8; 4];
    let mut response_version = [0u8; 4];

    stream.read_exact(&mut response_magic)?;
    stream.read_exact(&mut response_version)?;

    if response_magic != ETHEREUM_MAGIC {
        return Err("Invalid magic bytes in response".into());
    }

    let remote_version = u32::from_be_bytes(response_version);

    // ... (Process the response, perform additional verification if needed)

    println!("Ethereum handshake successful. Remote version: {}", remote_version);

    Ok(())
}


//main function
fn main() {
   //here handshake function will be called
   if let Err(err) = perform_ethereum_handshake("127.0.0.1:30303") {
        eprintln!("Error: {}", err);
    }
}
