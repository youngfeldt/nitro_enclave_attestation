use nitro_enclaves_attestation_doc::{get_attestation_doc, AttestationDocument};
use std::io::Write;
use std::net::Shutdown;
use vsock::VsockStream;
use std::time::Duration;
use std::thread;

const VSOCK_PORT: u32 = 5005; // The port your host program will listen on
const SLEEP_DURATION: Duration = Duration::from_secs(60); // 1 minute in seconds

fn get_attestation() -> Result<AttestationDocument, Box<dyn std::error::Error>> {
    let doc = get_attestation_doc()?;
    Ok(doc)
}

fn send_attestation_over_vsock(attestation: AttestationDocument) -> Result<(), Box<dyn std::error::Error>> {
    // Establish VSOCK connection
    let mut stream = VsockStream::connect(3, VSOCK_PORT)?; // CID 3 represents the host

    // Serialize the attestation document to JSON
    let attestation_json = serde_json::to_string(&attestation)?;

    // Send the serialized document over vsock
    stream.write_all(attestation_json.as_bytes())?;
    
    // Close the connection
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn main() {
    // Start an infinite loop to send the attestation document once every minute
    loop {
        // Retrieve the attestation document
        let attestation_document = match get_attestation() {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Failed to get attestation document: {:?}", e);
                continue; // Skip sending and try again next iteration
            }
        };

        // Send the attestation document to the host through vsock
        match send_attestation_over_vsock(attestation_document) {
            Ok(_) => println!("Attestation document sent successfully"),
            Err(e) => eprintln!("Failed to send attestation document: {:?}", e),
        }

        // Sleep for 1 minute
        thread::sleep(SLEEP_DURATION);
    }
}

