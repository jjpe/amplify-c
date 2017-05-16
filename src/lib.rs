extern crate libc;
extern crate libcereal;

use libc::{c_char, c_int};
use libcereal::*;
use libcereal::amplify::*;
use std::ffi::CStr;

pub const SER_METHOD_CAPNP: c_int = 0x0;
pub const SER_METHOD_JSON: c_int = 0x1;


#[no_mangle]
pub extern "C" fn uclient_new() -> *mut UClient {
    Box::into_raw(Box::new(UClient::new().unwrap(/* TODO: ClientErr */)))
}

#[no_mangle]
pub extern "C" fn uclient_set_ser_method(client: *mut UClient, method: c_int) {
    let m = match method {
        SER_METHOD_CAPNP => Method::CapnProto,
        SER_METHOD_JSON => Method::Json,
        _ => panic!("Unknown serialization method: {}", method),
    };
    unsafe { (*client).set_serialization_method(m); }
}

#[no_mangle]
pub extern "C" fn uclient_set_rx_addr(client: *mut UClient, addr: *const c_char) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = unsafe { CStr::from_ptr(addr) };
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    unsafe { (*client).set_receive_addr(addr); }
}

#[no_mangle]
pub extern "C" fn uclient_set_tx_addr(client: *mut UClient, addr: *const c_char) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = unsafe { CStr::from_ptr(addr) };
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    unsafe { (*client).set_send_addr(addr); }
}

#[no_mangle]
pub extern "C" fn uclient_connect(client: *mut UClient) -> *mut CClient {
    let client: Box<UClient> = unsafe { Box::from_raw(client) };
    let client: CClient = client.connect().unwrap(/* TODO: ClientErr */);
    Box::into_raw(Box::new(client))
}

#[no_mangle]
pub extern "C" fn uclient_destroy(client: *mut UClient) {
    unsafe { drop(Box::from_raw(client)) }
}



#[no_mangle]
pub extern "C" fn cclient_send(client: *mut CClient, msg: *const Msg) {
    unsafe { (*client).send(&*msg).unwrap(/* TODO: ClientErr */) }
}

#[no_mangle]
pub extern "C" fn cclient_receive(client: *mut CClient, msg: *mut Msg) {
    unsafe { (*client).receive(&mut *msg).unwrap(/* TODO: ClientErr */) }
}

#[no_mangle]
pub extern "C" fn cclient_destroy(client: *mut CClient) {
    unsafe { drop(Box::from_raw(client)) }
}



// TODO: Decide if a C API for the Broadcaster types is even needed.



#[cfg(test)]
mod tests {
    use *;
    use std::io;
    use std::io::Write;
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn basic_toolgraph_test() {
        let (broadcaster_tx, broadcaster_rx) = mpsc::channel::<Msg>();
        let (source_tx, source_rx) = mpsc::channel::<Msg>();
        let (sink1_tx, sink1_rx) = mpsc::channel::<Msg>();
        let (sink2_tx, sink2_rx) = mpsc::channel::<Msg>();
        let (sink3_tx, sink3_rx) = mpsc::channel::<Msg>();

        thread::spawn(move || {
            let mut broadcaster: BBroadcaster<Msg> = UBroadcaster::new().unwrap()
                .bind().unwrap();

            broadcaster.receive().unwrap();
            broadcaster.broadcast().unwrap();
            broadcaster_tx.send(broadcaster.buffer_ref().clone()).unwrap();
        });

        thread::spawn(move || {
            let sink = uclient_new();
            let sink = uclient_connect(sink);
            let mut msg = Msg::default();

            cclient_receive(sink, &mut msg);
            println!("sink1 received msg");
            io::stdout().flush().unwrap();
            sink1_tx.send(msg).unwrap();
        });

        thread::spawn(move || {
            let sink = uclient_new();
            let sink = uclient_connect(sink);
            let mut msg = Msg::default();

            cclient_receive(sink, &mut msg);
            println!("sink2 received msg");
            io::stdout().flush().unwrap();
            sink2_tx.send(msg).unwrap();
        });

        thread::spawn(move || {
            let sink = uclient_new();
            let sink = uclient_connect(sink);
            let mut msg = Msg::default();

            cclient_receive(sink, &mut msg);
            println!("sink3 received msg");
            io::stdout().flush().unwrap();
            sink3_tx.send(msg).unwrap();
        });

        // Initialize the source last, so that the entire network has already
        // settled once the Source sends the message.
        thread::spawn(move || {
            let source = uclient_new();
            let source = uclient_connect(source);
            let mut msg = Msg::default();
            *msg.contents_mut() = Some(Contents::Text(String::from("Lorem Ipsum")));
            *msg.request_number_mut() = 42;
            *msg.origin_mut() = Some(String::from("C API test Source"));

            thread::sleep(Duration::from_millis(1000));
            cclient_send(source, &msg);
            source_tx.send(msg).unwrap();
        });

        let bmsg = broadcaster_rx.recv().unwrap();
        let smsg = source_rx.recv().unwrap();
        let s1msg = sink1_rx.recv().unwrap();
        let s2msg = sink2_rx.recv().unwrap();
        let s3msg = sink3_rx.recv().unwrap();

        assert_eq!(smsg, bmsg, "{:#?} != {:#?}", smsg, bmsg);
        assert_eq!(bmsg, s1msg, "{:#?} != {:#?}", bmsg, s1msg);
        assert_eq!(bmsg, s2msg, "{:#?} != {:#?}", bmsg, s2msg);
        assert_eq!(bmsg, s3msg, "{:#?} != {:#?}", bmsg, s3msg);
    }
}

//  LocalWords:  cclient, uclient
