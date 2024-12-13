use std::process::{Command, Stdio};

fn get_password_from_keychain() -> String{
    let output = Command::new("security")
    .args(["find-generic-password", "-a", "mark", "-s", "OpenConnectVPN", "-w"])
    .output()
    .expect("Failed to retrieve password from the Keychain");

    String::from_utf8(output.stdout)
    .expect("Failed to convert password to string")
    .trim()
    .to_string()
}


fn main() {
    let vpn_password = get_password_from_keychain();

    // Command to start OpenConnect with sudo
    let mut child = Command::new("sudo")
        .arg("openconnect")
        .arg("96.75.117.5")
        .arg("--servercert")
        .arg("pin-sha256:I4yaHHsEa/Ur2vMhlbm5CpBmDKzLNNRx7+JwmTjLXnk=")
        .arg("-u")
        .arg("mark")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start openconnect process");

    // Write the VPN password to openconnect stdin
    if let Some(ref mut stdin) = child.stdin {
        use std::io::Write;
        stdin.write_all(vpn_password.as_bytes()).expect("Failed to write password to openconnect stdin");
    }

    //wait for the open connect process to complete
    let output = child.wait_with_output().expect("Failed to read openconnect output");
    println!("OpenConnect exited with status: {}", output.status);

}
