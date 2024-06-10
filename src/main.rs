
use std::{io::BufRead, net::*, str::from_boxed_utf8_unchecked};
fn main() {
 // Ipv4Addr::new(127, 0, 0, 1);

let ip ="127.0.0.1".parse::<Ipv4Addr>().unwrap();
let port = 1234;

// Socket object 
let serversocket= SocketAddrV4::new(ip, port);

let listener =TcpListener::bind(serversocket).unwrap();

for clientserversocket: in listener.incoming();


   let mut clientsocketconnection =match  clientserversocket {
    Ok(s) => s,
    Err(e) => {
        println!("{}",e);
        continue;
    }
    

   };


println!("[+] connection recieved: {}:{}".clientsocketconnection.peer_add().unwrap().ip(),clientsocketconnection.peer_addr().unwrap().port());
loop {
// sending the messsage \
//userinut contains the \n\r 
let mut userinput:String = String::new();
let flag =userinput.clone();
std::io::stdin().read_line(&mut userinput).unwrap();


//let mut  msg: Vec<u8> = "Hello there".bytes().collect::<Vec<u8>>
let mut userinput = userinput.trim_end_matches("\r\n");
let mut  userbytes=userinput.bytes().collect::<Vec<u8>>();
let bytessent = clientsocketconnection.write(&mut userbytes);
println!("bytes sent: {}",bytessent.unwrap().to_string());


// if userinput is the quit the break the loop

if flag.trim_end_matches("\r\n") == "quit" {
    break;
}




// receiving the msg 

let mut buffer:Vec<u8> = vec![0; 4096];
clientsocketconnection.read(&mut buffer);

println!("received from: {}:{} -> {}",clientsocketconnection.peer_add().unwrap().ip(),clientsocketconnection.peer_addr().unwrap().port(),String::from_utf8_lossy(&buffer));




clientsocketconnection.shutdown(Shutdown::Both);








};
}
