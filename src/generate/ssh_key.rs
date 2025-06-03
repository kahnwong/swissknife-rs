use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

fn format_private_key(key: &SigningKey) -> String {
    use base64::{Engine as _, engine::general_purpose};

    let private_bytes = key.to_bytes();
    let public_bytes = key.verifying_key().to_bytes();

    // Simple OpenSSH format
    let mut data = Vec::new();
    data.extend_from_slice(b"openssh-key-v1\0");
    data.extend_from_slice(&4u32.to_be_bytes());
    data.extend_from_slice(b"none");
    data.extend_from_slice(&4u32.to_be_bytes());
    data.extend_from_slice(b"none");
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&1u32.to_be_bytes());

    // Public key section
    let mut pub_section = Vec::new();
    pub_section.extend_from_slice(&11u32.to_be_bytes());
    pub_section.extend_from_slice(b"ssh-ed25519");
    pub_section.extend_from_slice(&32u32.to_be_bytes());
    pub_section.extend_from_slice(&public_bytes);

    data.extend_from_slice(&(pub_section.len() as u32).to_be_bytes());
    data.extend_from_slice(&pub_section);

    // Private key section
    let mut priv_section = Vec::new();
    priv_section.extend_from_slice(&[1, 2, 3, 4, 1, 2, 3, 4]); // checksum
    priv_section.extend_from_slice(&11u32.to_be_bytes());
    priv_section.extend_from_slice(b"ssh-ed25519");
    priv_section.extend_from_slice(&32u32.to_be_bytes());
    priv_section.extend_from_slice(&public_bytes);
    priv_section.extend_from_slice(&64u32.to_be_bytes());
    priv_section.extend_from_slice(&private_bytes);
    priv_section.extend_from_slice(&public_bytes);
    priv_section.extend_from_slice(&0u32.to_be_bytes()); // empty comment

    // Padding
    while priv_section.len() % 8 != 0 {
        priv_section.push((priv_section.len() % 8 + 1) as u8);
    }

    data.extend_from_slice(&(priv_section.len() as u32).to_be_bytes());
    data.extend_from_slice(&priv_section);

    let b64 = general_purpose::STANDARD.encode(&data);
    format!(
        "-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----\n",
        b64.chars()
            .collect::<Vec<_>>()
            .chunks(70)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn format_public_key(key: &VerifyingKey) -> String {
    use base64::{Engine as _, engine::general_purpose};

    let public_bytes = key.to_bytes();
    let mut data = Vec::new();

    data.extend_from_slice(&11u32.to_be_bytes());
    data.extend_from_slice(b"ssh-ed25519");
    data.extend_from_slice(&32u32.to_be_bytes());
    data.extend_from_slice(&public_bytes);

    format!("ssh-ed25519 {}", general_purpose::STANDARD.encode(&data))
}

fn write_string_to_file(file_path: String, data: String, permission: u32) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(permission)
        .open(file_path)
        .unwrap();

    file.write_all(data.as_bytes()).unwrap();
}

fn return_key_path(file_name: &str) -> String {
    let current_dir = env::current_dir().unwrap_or_else(|err| {
        // In a real application, you'd log this error using a proper logging crate
        // like `log::error!` or `tracing::error!`.
        eprintln!("Failed to get current directory: {}", err);
        panic!("Failed to get current directory"); // Exits the program like log.Fatal
    });

    let mut key_path = current_dir;
    key_path.push(file_name);

    // Convert PathBuf to a String, append ".pem", and then convert back to String
    // This part is a bit more manual than Go's Sprintf for paths.
    let mut key_path_string = key_path.to_string_lossy().into_owned();
    key_path_string.push_str(".pem");

    key_path_string
}

pub fn ssh_key(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random Ed25519 private key
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);

    // Derive the public key from the private key
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    // write to file
    let private_ssh = format_private_key(&signing_key);
    let public_ssh = format_public_key(&verifying_key);

    let private_key_filename = format!("{}.pem", name);
    write_string_to_file(private_key_filename, private_ssh, 0o600);

    let public_key_filename = format!("{}.pub", name);
    write_string_to_file(public_key_filename, public_ssh, 0o644);

    println!("SSH key created at: {}", return_key_path(name));

    Ok(())
}
