extern crate libc;
extern crate libcereal;

use libc::{c_char, c_int, c_uchar, c_ulonglong };
use libcereal::*;
use libcereal::amplify::*;
use std::ffi::{CStr, CString};
use std::mem;

pub const SER_METHOD_CAPNP: c_int = 0x0;
pub const SER_METHOD_JSON: c_int = 0x1;

/// NOTE: The getters usually return a (*const T) borrow into the data, whereas
/// the setters consume their (*mut T) argument. This is important with regards
/// to memory safety.

/******************************************************************************/
/*                              UClient                                       */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn uclient_new() -> *mut UClient {
    Box::into_raw(Box::new(UClient::new().unwrap(/* TODO: ClientErr */)))
}

#[no_mangle]
pub extern "C" fn uclient_destroy(client: *mut UClient) {
    unsafe { drop(Box::from_raw(client)) }
}

#[no_mangle]
pub extern "C" fn uclient_serialize_using_capn_proto(client: *mut UClient) {
    unsafe { (*client).set_serialization_method(Method::CapnProto); }
}

#[no_mangle]
pub extern "C" fn uclient_serialize_using_json(client: *mut UClient) {
    unsafe { (*client).set_serialization_method(Method::Json); }
}

#[no_mangle]
pub extern "C" fn uclient_set_rx_addr(client: *mut UClient, addr: *const c_uchar) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = unsafe { CStr::from_ptr(addr as *const c_char) };
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    unsafe { (*client).set_receive_addr(addr); }
}

#[no_mangle]
pub extern "C" fn uclient_set_tx_addr(client: *mut UClient, addr: *const c_uchar) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = unsafe { CStr::from_ptr(addr as *const c_char) };
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    unsafe { (*client).set_send_addr(addr); }
}

#[no_mangle]
pub extern "C" fn uclient_set_rx_timeout(client: *mut UClient, timeout_ms: c_int) {
    let timeout = match timeout_ms {
        -1 => Timeout::Block,
        0 => Timeout::None,
        millis => Timeout::Millis(millis as usize),
    };
    unsafe { (*client).set_receive_timeout(timeout); }
}

#[no_mangle]
pub extern "C" fn uclient_set_tx_timeout(client: *mut UClient, timeout_ms: c_int) {
    let timeout = match timeout_ms {
        -1 => Timeout::Block,
        0 => Timeout::None,
        millis => Timeout::Millis(millis as usize),
    };
    unsafe { (*client).set_send_timeout(timeout); }
}

#[no_mangle]
pub extern "C" fn uclient_connect(client: *mut UClient) -> *mut CClient {
    let client: Box<UClient> = unsafe { Box::from_raw(client) };
    let client: CClient = client.connect().unwrap(/* TODO: ClientErr */);
    Box::into_raw(Box::new(client))
}

/******************************************************************************/
/*                              CClient                                       */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn cclient_destroy(client: *mut CClient) {
    unsafe { drop(Box::from_raw(client)) }
}

#[no_mangle]
pub extern "C" fn cclient_set_rx_timeout(client: *mut CClient, timeout_ms: c_int) {
    let timeout = match timeout_ms {
        -1 => Timeout::Block,
        0 => Timeout::None,
        millis => Timeout::Millis(millis as usize),
    };
    unsafe { (*client).set_receive_timeout(timeout); }
}

#[no_mangle]
pub extern "C" fn cclient_set_tx_timeout(client: *mut CClient, timeout_ms: c_int) {
    let timeout = match timeout_ms {
        -1 => Timeout::Block,
        0 => Timeout::None,
        millis => Timeout::Millis(millis as usize),
    };
    unsafe { (*client).set_send_timeout(timeout); }
}

#[no_mangle]
pub extern "C" fn cclient_send(client: *mut CClient, msg: *const Msg) {
    unsafe { (*client).send(&*msg).unwrap(/* TODO: ClientErr */) }
}

#[no_mangle]
pub extern "C" fn cclient_receive(client: *mut CClient, msg: *mut Msg) {
    unsafe { (*client).receive(&mut *msg).unwrap(/* TODO: ClientErr */) }
}

/******************************************************************************/
/*                              Msg                                           */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn msg_new() -> *mut Msg {
    Box::into_raw(Box::new(Msg::default()))
}

#[no_mangle]
pub extern "C" fn msg_destroy(msg: *mut Msg) {
    unsafe { drop(Box::from_raw(msg)) }
}

#[no_mangle]
pub extern "C" fn msg_set_source(msg: *mut Msg, source: *const c_uchar) {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(!source.is_null(), "source must not be null");
    let string: String = c_string_to_str(source).to_owned();
    unsafe { *(*msg).source_mut() = string; }
}

#[no_mangle]
pub extern "C" fn msg_get_source(msg: *mut Msg) -> *const c_uchar {
    assert!(!msg.is_null(), "msg must not be null");
    let source: &str = unsafe { (*msg).source_ref() };
    str_to_c_string(source) as *const c_uchar
}

#[no_mangle]
pub extern "C" fn msg_set_request_number(msg: *mut Msg, reqno: c_ulonglong) {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { *(*msg).request_number_mut() = reqno; }
}

#[no_mangle]
pub extern "C" fn msg_get_request_number(msg: *mut Msg) -> c_ulonglong {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { (*msg).request_number() }
}

#[no_mangle]
pub extern "C" fn msg_set_origin(msg: *mut Msg, origin: *const c_uchar) {
    assert!(!msg.is_null(), "msg must not be null");
    let origin =
        if origin.is_null() { None }
        else { Some(c_string_to_str(origin).to_owned()) };
    unsafe { *(*msg).origin_mut() = origin; }
}

#[no_mangle]
pub extern "C" fn msg_get_origin(msg: *mut Msg) -> *const c_uchar {
    assert!(!msg.is_null(), "msg must not be null");
    match unsafe { (*msg).origin_ref() } {
        None => std::ptr::null(),
        Some(origin) => str_to_c_string(origin) as *const c_uchar,
    }
}

#[no_mangle]
pub extern "C" fn msg_set_contents(msg: *mut Msg, contents: *mut Contents) {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { *(*msg).contents_mut() = consume_raw(contents); }
}

#[no_mangle]
pub extern "C" fn msg_get_contents(msg: *mut Msg) -> *const Contents {
    assert!(!msg.is_null(), "msg must not be null");
    match unsafe { (*msg).contents_ref() } {
        None => std::ptr::null(),
        Some(contents) => contents as *const Contents,
    }
}

#[no_mangle]
pub extern "C" fn msg_add_region(msg: *mut Msg, region: *mut Region) {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(!region.is_null(), "region must not be null");
    unsafe { (*msg).regions_mut().push(*Box::from_raw(region)); }
}

#[no_mangle]
pub extern "C" fn msg_clear_regions(msg: *mut Msg) {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { (*msg).regions_mut().clear(); }
}

#[no_mangle]
pub extern "C" fn msg_get_region(msg: *mut Msg, index: c_ulonglong)
                                 -> *const Region {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(index < msg_count_regions(msg), "index out of bounds");
    let regions = unsafe { (*msg).regions_ref() };
    &regions[index as usize]
}

#[no_mangle]
pub extern "C" fn msg_count_regions(msg: *mut Msg) -> c_ulonglong {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { (*msg).regions_ref().len() as c_ulonglong }
}

#[no_mangle]
pub extern "C" fn msg_set_language(msg: *mut Msg, lang: *mut Language) {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { *(*msg).language_mut() = consume_raw(lang); }
}

#[no_mangle]
pub extern "C" fn msg_get_language(msg: *mut Msg) -> *const Language {
    assert!(!msg.is_null(), "msg must not be null");
    match unsafe { (*msg).language_ref() } {
        None => std::ptr::null(),
        Some(lang) => lang,
    }
}

#[no_mangle]
pub extern "C" fn msg_set_ast(msg: *mut Msg, ast: *mut Ast) {
    assert!(!msg.is_null(), "msg must not be null");
    unsafe { *(*msg).ast_mut() = consume_raw(ast); }
}

#[no_mangle]
pub extern "C" fn msg_get_ast(msg: *mut Msg) -> *const Ast {
    assert!(!msg.is_null(), "msg must not be null");
    match unsafe { (*msg).ast_ref() } {
        None => std::ptr::null(),
        Some(ast) => ast,
    }
}

/******************************************************************************/
/*                              Contents                                      */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn contents_new_text(text: *const c_uchar) -> *mut Contents {
    let string: &str = c_string_to_str(text);
    Box::into_raw(Box::new(Contents::from(string)))
}

#[no_mangle]
pub extern "C" fn contents_new_entries() -> *mut Contents {
    Box::into_raw(Box::new(Contents::from(vec![])))
}

#[no_mangle]
pub extern "C" fn contents_destroy(contents: *mut Contents) {
    assert!(!contents.is_null(), "contents must not be null");
    unsafe { drop(Box::from_raw(contents)) }
}

#[no_mangle]
pub extern "C" fn contents_is_text(contents: *mut Contents) -> c_int {
    if contents.is_null() {  return false as c_int;  }
    match unsafe { &*contents } {
        &Contents::Text(_) => true as c_int,
        _ => false as c_int,
    }
}

#[no_mangle]
pub extern "C" fn contents_add_text(contents: *mut Contents, text: *const c_uchar) {
    assert!(!contents.is_null(), "contents must not be null");
    assert!(!text.is_null(), "text must not be null");
    match unsafe { &mut *contents } {
        &mut Contents::Text(ref mut t) => t.push_str(c_string_to_str(text)),
        c => panic!("Can only add text to Contents::Text, but found {:?}", c),
    }
}

#[no_mangle]
pub extern "C" fn contents_get_text(contents: *mut Contents) -> *const c_uchar {
    assert!(!contents.is_null(), "contents must not be null");
    match unsafe { &*contents } {
        &Contents::Text(ref text) =>
            str_to_c_string(text.as_str()) as *const c_uchar,
        c => panic!("Expected Contents::Text, but found {:?}", c),
    }
}

#[no_mangle]
pub extern "C" fn contents_is_entries(contents: *mut Contents) -> c_int {
    if contents.is_null() {  return false as c_int;  }
    match unsafe { &*contents } {
        &Contents::Entries(_) => true as c_int,
        _ => false as c_int,
    }
}

#[no_mangle]
pub extern "C" fn contents_add_entry(contents: *mut Contents, entry: *const c_uchar) {
    assert!(!contents.is_null(), "contents must not be null");
    assert!(!entry.is_null(), "entry must not be null");
    let entry: &str = c_string_to_str(entry);
    match unsafe { &mut *contents } {
        &mut Contents::Entries(ref mut vec) => vec.push(String::from(entry)),
        c => panic!("Can only add an entry to Contents::Entries, but found {:?}", c),
    }
}

#[no_mangle]
pub extern "C" fn contents_get_entry(contents: *mut Contents, index: c_ulonglong)
                                     -> *const c_uchar {
    assert!(!contents.is_null(), "contents must not be null");
    match unsafe { &*contents } {
        &Contents::Entries(ref entries) => {
            let num_entries = contents_count_entries(contents);
            assert!(index < num_entries, "index out of bounds: {} >= {}", index, num_entries);
            let string: &String = entries.get(index as usize).unwrap(/* TODO: None */);
            str_to_c_string(string.as_str()) as *const c_uchar
        },
        c => panic!("Expected Contents::Entries, but found {:?}", c),
    }
}

#[no_mangle]
pub extern "C" fn contents_count_entries(contents: *mut Contents) -> c_ulonglong {
    assert!(!contents.is_null(), "contents must not be null");
    match unsafe { &*contents } {
        &Contents::Entries(ref entries) => entries.len() as c_ulonglong,
        c => panic!("Expected Contents::Entries, but found {:?}", c),
    }
}

/******************************************************************************/
/*                              Region                                        */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn region_new(begin: c_ulonglong, end: c_ulonglong) -> *mut Region {
    Box::into_raw(Box::new(Region { begin: begin, end: end }))
}

#[no_mangle]
pub extern "C" fn region_destroy(region: *mut Region) {
    assert!(!region.is_null(), "region must not be null");
    unsafe { drop(Box::from_raw(region)) }
}

#[no_mangle]
pub extern "C" fn region_get_begin(region: *const Region) -> c_ulonglong {
    assert!(!region.is_null(), "region must not be null");
    unsafe { (*region).begin }
}

#[no_mangle]
pub extern "C" fn region_get_end(region: *const Region) -> c_ulonglong {
    assert!(!region.is_null(), "region must not be null");
    unsafe { (*region).end }
}

/******************************************************************************/
/*                              Language                                      */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn language_new(language: *const c_uchar) -> *mut Language {
    assert!(!language.is_null(), "language must not be null");
    let string: &str = c_string_to_str(language);
    Box::into_raw(Box::new(Language::from(string)))
}

#[no_mangle]
pub extern "C" fn language_destroy(language: *mut Language) {
    assert!(!language.is_null(), "language must not be null");
    unsafe { drop(Box::from_raw(language)) }
}

#[no_mangle]
pub extern "C" fn language_set_name(language: *mut Language, new: *const c_uchar) {
    assert!(!language.is_null(), "language must not be null");
    assert!(!new.is_null(), "new must not be null");
    let new: &str = c_string_to_str(new);
    unsafe { *(*language).as_mut() = String::from(new); }
}

#[no_mangle]
pub extern "C" fn language_get_name(language: *mut Language) -> *const c_char {
    let lang: &str =
        if language.is_null() { "" }
        else { unsafe { (*language).as_ref() } };
    str_to_c_string(lang)
}

/******************************************************************************/
/*                              Ast                                           */
/******************************************************************************/
#[no_mangle]
pub extern "C" fn ast_new(name: *const c_uchar) -> *mut Ast {
    let ast = Ast::new(c_string_to_str(name));
    Box::into_raw(Box::new(ast))
}

#[no_mangle]
pub extern "C" fn ast_destroy(ast: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    unsafe { drop(Box::from_raw(ast)) }
}

#[no_mangle]
pub extern "C" fn ast_get_name(ast: *mut Ast) -> *const c_uchar {
    assert!(!ast.is_null(), "ast must not be null");
    let name = unsafe { (*ast).name_ref() };
    str_to_c_string(name) as *const c_uchar
}

#[no_mangle]
pub extern "C" fn ast_set_data(ast: *mut Ast, data: *const c_uchar) {
    assert!(!ast.is_null(), "ast must not be null");
    assert!(!data.is_null(), "data must not be null");
    let data: &str = c_string_to_str(data);
    unsafe { *(*ast).data_mut() = String::from(data); }
}

#[no_mangle]
pub extern "C" fn ast_get_data(ast: *mut Ast) -> *const c_uchar {
    assert!(!ast.is_null(), "ast must not be null");
    let data = unsafe { (*ast).data_ref() };
    str_to_c_string(data) as *const c_uchar
}

#[no_mangle]
pub extern "C" fn ast_clear_data(ast: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    unsafe { (*ast).data_mut().clear(); }
}

#[no_mangle]
pub extern "C" fn ast_add_child(ast: *mut Ast, child: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    assert!(!child.is_null(), "child must not be null");
    unsafe { (*ast).children_mut().push(*Box::from_raw(child)); }
}

#[no_mangle]
pub extern "C" fn ast_get_child(ast: *mut Ast, index: c_ulonglong) -> *const Ast {
    assert!(!ast.is_null(), "ast must not be null");
    let num_children = ast_count_children(ast);
    assert!(index < num_children, "index out of bounds: {} >= {}", index, num_children);
    let children = unsafe { (*ast).children_ref() };
    &children[index as usize]
}

#[no_mangle]
pub extern "C" fn ast_clear_children(ast: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    unsafe { (*ast).children_mut().clear(); }
}

#[no_mangle]
pub extern "C" fn ast_count_children(ast: *mut Ast) -> c_ulonglong {
    assert!(!ast.is_null(), "ast must not be null");
    unsafe { (*ast).children_ref().len() as c_ulonglong }
}

/******************************************************************************/
/*                              utility                                       */
/******************************************************************************/
fn c_string_to_str<'s>(cstring: *const c_uchar) -> &'s str {
    let cstr = unsafe { CStr::from_ptr(cstring as *const c_char) };
    cstr.to_str().unwrap(/* TODO: str::Utf8Error */)
}

fn str_to_c_string(s: &str) -> *const c_char {
    let cstring = CString::new(s).unwrap(/* TODO: NulError */);
    let ptr = cstring.as_ptr();
    mem::forget(cstring);
    ptr
}

unsafe fn consume_raw<T>(ptr: *mut T) -> Option<T> {
    if ptr.is_null() {  None  } else {  Some(*Box::from_raw(ptr))  }
}

// unsafe fn borrow_option<T>(option: &Option<&T>) -> *const T {
//     match *option {
//         None => std::ptr::null(),
//         Some(t) => t,
//     }
// }

// unsafe fn consume_option<T>(option: Option<T>) -> *mut T {
//     match *option {
//         None => std::ptr::null(),
//         Some(t) => &mut t, // TODO: is this even correct?
//     }
// }



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
