use std::net::UdpSocket;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "romans-kleines-programm-zum-abspielen-von-avi-videos",
    about = "Für Roman"
)]
struct Opt {
    /// Message to write to BrightSign screen via UDP.
    #[structopt(short = "m", long = "message")]
    message: String,

    /// IP Address of BrightSign screen.
    #[structopt(short = "e", long = "endpoint", default_value = "10.0.1.4:5000")]
    endpoint: String,
}

fn main() {
    let opt = Opt::from_args();
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .connect(opt.endpoint)
        .expect("connect function failed");
    socket
        .send(opt.message.as_bytes())
        .expect("couldn't send message");
    println!("PUPSI");
}
