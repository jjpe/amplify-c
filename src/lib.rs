extern crate libc;
extern crate amplify;

use libc::{c_char, c_int, c_uint, c_uchar, c_ulonglong };
use amplify::*;
use amplify::amplify::*;
use std::ffi::{CStr, CString};
use std::mem;

/// NOTE: The getters usually return a (*const T) borrow into the data, whereas
/// the setters consume their (*mut T) argument. This is important with regards
/// to memory safety.

/******************************************************************************/
/*                              UReporter                                     */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn ureporter_new() -> *mut UReporter {
    Box::into_raw(Box::new(UReporter::new().unwrap(/* TODO: ReportErr */)))
}


#[no_mangle]
pub unsafe extern "C"
fn ureporter_destroy(sender: *mut UReporter) {
    drop(Box::from_raw(sender))
}

#[no_mangle]
pub unsafe extern "C"
fn ureporter_serialize_using_capn_proto(ureporter: *mut UReporter) {
    (*ureporter).set_serialization_method(Method::CapnProto);
}

#[no_mangle]
pub unsafe extern "C"
fn ureporter_serialize_using_json(ureporter: *mut UReporter) {
    (*ureporter).set_serialization_method(Method::Json);
}

#[no_mangle]
pub unsafe extern "C"
fn ureporter_set_tx_addr(ureporter: *mut UReporter, addr: *const c_uchar) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = CStr::from_ptr(addr as *const c_char);
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    (*ureporter).set_send_addr(&addr);
}

#[no_mangle]
pub unsafe extern "C"
fn ureporter_set_tx_timeout(ureporter: *mut UReporter, timeout_ms: c_int) {
    let timeout = Timeout::from_number(timeout_ms as isize);
    (*ureporter).set_send_timeout(timeout);
}

#[no_mangle]
pub unsafe extern "C"
fn ureporter_set_tx_hwm(ureporter: *mut UReporter, hwm: c_uint) {
    (*ureporter).set_send_hwm(Hwm::from_number(hwm as usize));
}

#[no_mangle]
pub unsafe extern "C"
fn ureporter_connect(ureporter: *mut UReporter) -> *mut CReporter {
    let ureporter: Box<UReporter> = Box::from_raw(ureporter);
    let creporter: CReporter = ureporter.connect().unwrap(/* TODO: ReportErr */);
    Box::into_raw(Box::new(creporter))
}


/******************************************************************************/
/*                              CReporter                                     */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn creporter_destroy(creporter: *mut CReporter) {
    drop(Box::from_raw(creporter))
}

#[no_mangle]
pub unsafe extern "C"
fn creporter_send(creporter: *mut CReporter, report: *const Report) {
    (*creporter).send(&*report).unwrap(/* TODO: ReportErr */)
}

#[no_mangle]
pub unsafe extern "C"
fn creporter_set_tx_timeout(creporter: *mut CReporter, timeout_ms: c_int) {
    let timeout = Timeout::from_number(timeout_ms as isize);
    (*creporter).set_send_timeout(timeout).unwrap(/* TODO: ReportErr */);
}

#[no_mangle]
pub unsafe extern "C"
fn creporter_set_tx_hwm(creporter: *mut CReporter, hwm: c_uint) {
    let hwm = Hwm::from_number(hwm as usize);
    (*creporter).set_send_hwm(hwm).unwrap(/* TODO: ReportErr */);
}


/******************************************************************************/
/*                              Report                                        */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn report_new() -> *mut Report {
    Box::into_raw(Box::new(Report::default()))
}

#[no_mangle]
pub unsafe extern "C"
fn report_destroy(report: *mut Report) {
    drop(Box::from_raw(report))
}

#[no_mangle]
pub unsafe extern "C"
fn report_set_action(report: *mut Report, action: *const c_uchar) {
    assert!(!report.is_null(), "report must not be null");
    assert!(!action.is_null(), "action must not be null");
    *(*report).action_mut() = c_string_to_str(action).to_owned();
}

#[no_mangle]
pub unsafe extern "C"
fn report_get_action(report: *mut Report) -> *const c_uchar {
    assert!(!report.is_null(), "report must not be null");
    let action: &str = (*report).action_ref();
    str_to_c_string(action) as *const c_uchar
}

#[no_mangle]
pub unsafe extern "C"
fn report_set_process(report: *mut Report, process: *const c_uchar) {
    assert!(!report.is_null(), "report must not be null");
    assert!(!process.is_null(), "process must not be null");
    *(*report).process_mut() = c_string_to_str(process).to_owned();
}

#[no_mangle]
pub unsafe extern "C"
fn report_get_process(report: *mut Report) -> *const c_uchar {
    assert!(!report.is_null(), "report must not be null");
    let process: &str = (*report).process_ref();
    str_to_c_string(process) as *const c_uchar
}

#[no_mangle]
pub unsafe extern "C"
fn report_set_request_number(report: *mut Report, reqno: c_ulonglong) {
    assert!(!report.is_null(), "report must not be null");
    *(*report).request_number_mut() = reqno;
}

#[no_mangle]
pub unsafe extern "C"
fn report_get_request_number(report: *mut Report) -> c_ulonglong {
    assert!(!report.is_null(), "report must not be null");
    (*report).request_number()
}

#[no_mangle]
pub unsafe extern "C"
fn report_set_duration_nanos(report: *mut Report, duration_nanos: c_ulonglong) {
    assert!(!report.is_null(), "report must not be null");
    *(*report).duration_nanos_mut() = duration_nanos;
}

#[no_mangle]
pub unsafe extern "C"
fn report_get_duration_nanos(report: *mut Report) -> c_ulonglong {
    assert!(!report.is_null(), "report must not be null");
    (*report).duration_nanos()
}

#[no_mangle]
pub unsafe extern "C"
fn report_set_command(report: *mut Report, command: *const c_uchar) {
    assert!(!report.is_null(), "report must not be null");
    *(*report).command_mut() =
        if command.is_null() { None }
        else { Some(c_string_to_str(command).to_owned()) };
}

#[no_mangle]
pub unsafe extern "C"
fn report_get_command(report: *mut Report) -> *const c_uchar {
    assert!(!report.is_null(), "report must not be null");
    match (*report).command_ref() {
        None => std::ptr::null(),
        Some(cmd) => str_to_c_string(cmd) as *const c_uchar,
    }
}


/******************************************************************************/
/*                              UClient                                       */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn uclient_new() -> *mut UClient {
    Box::into_raw(Box::new(UClient::new().unwrap(/* TODO: ClientErr */)))
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_destroy(client: *mut UClient) {
    drop(Box::from_raw(client))
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_serialize_using_capn_proto(client: *mut UClient) {
    (*client).set_serialization_method(Method::CapnProto);
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_serialize_using_json(client: *mut UClient) {
    (*client).set_serialization_method(Method::Json);
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_set_rx_addr(client: *mut UClient, addr: *const c_uchar) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = CStr::from_ptr(addr as *const c_char);
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    (*client).set_receive_addr(addr);
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_set_tx_addr(client: *mut UClient, addr: *const c_uchar) {
    if addr.is_null() {  panic!("addr must not be null");  }
    let cstr: &CStr = CStr::from_ptr(addr as *const c_char);
    let addr: &str = cstr.to_str().unwrap(/* TODO: str::UTf8Error */);
    let addr: Url = Url::parse(addr).unwrap(/* TODO: url::ParseError */);
    (*client).set_send_addr(addr);
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_set_rx_timeout(client: *mut UClient, timeout_ms: c_int) {
    let timeout = Timeout::from_number(timeout_ms as isize);
    (*client).set_receive_timeout(timeout);
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_set_tx_timeout(client: *mut UClient, timeout_ms: c_int) {
    let timeout = Timeout::from_number(timeout_ms as isize);
    (*client).set_send_timeout(timeout);
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_set_rx_hwm(client: *mut UClient, hwm: c_uint) {
    (*client).set_receive_hwm(Hwm::from_number(hwm as usize));
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_set_tx_hwm(client: *mut UClient, hwm: c_uint) {
    (*client).set_send_hwm(Hwm::from_number(hwm as usize));
}

#[no_mangle]
pub unsafe extern "C"
fn uclient_connect(client: *mut UClient) -> *mut CClient {
    let client: Box<UClient> = Box::from_raw(client);
    let client: CClient = client.connect().unwrap(/* TODO: ClientErr */);
    Box::into_raw(Box::new(client))
}

/******************************************************************************/
/*                              CClient                                       */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn cclient_destroy(client: *mut CClient) {
    drop(Box::from_raw(client))
}

#[no_mangle]
pub unsafe extern "C"
fn cclient_set_rx_timeout(client: *mut CClient, timeout_ms: c_int) {
    let timeout = Timeout::from_number(timeout_ms as isize);
    (*client).set_receive_timeout(timeout).unwrap(/* TODO: ClientErr */);
}

#[no_mangle]
pub unsafe extern "C"
fn cclient_set_tx_timeout(client: *mut CClient, timeout_ms: c_int) {
    let timeout = Timeout::from_number(timeout_ms as isize);
    (*client).set_send_timeout(timeout).unwrap(/* TODO: ClientErr */);
}

#[no_mangle]
pub unsafe extern "C"
fn cclient_set_rx_hwm(client: *mut CClient, hwm: c_uint) {
    let hwm = Hwm::from_number(hwm as usize);
    (*client).set_receive_hwm(hwm).unwrap(/* TODO: ClientErr */);
}

#[no_mangle]
pub unsafe extern "C"
fn cclient_set_tx_hwm(client: *mut CClient, hwm: c_uint) {
    let hwm = Hwm::from_number(hwm as usize);
    (*client).set_send_hwm(hwm).unwrap(/* TODO: ClientErr */);
}

#[no_mangle]
pub unsafe extern "C"
fn cclient_send(client: *mut CClient, msg: *const Msg) {
    (*client).send(&*msg).unwrap(/* TODO: ClientErr */);
}

#[no_mangle]
pub unsafe extern "C"
fn cclient_receive(client: *mut CClient, msg: *mut Msg) {
    (*client).receive(&mut *msg).unwrap(/* TODO: ClientErr */);
}

/******************************************************************************/
/*                              Msg                                           */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn msg_new() -> *mut Msg {
    Box::into_raw(Box::new(Msg::default()))
}

#[no_mangle]
pub unsafe extern "C"
fn msg_destroy(msg: *mut Msg) {
    drop(Box::from_raw(msg))
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_process(msg: *mut Msg, process: *const c_uchar) {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(!process.is_null(), "process must not be null");
    let string: String = c_string_to_str(process).to_owned();
    *(*msg).process_mut() = string;
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_process(msg: *mut Msg) -> *const c_uchar {
    assert!(!msg.is_null(), "msg must not be null");
    let process: &str = (*msg).process_ref();
    str_to_c_string(process) as *const c_uchar
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_request_number(msg: *mut Msg, reqno: c_ulonglong) {
    assert!(!msg.is_null(), "msg must not be null");
    *(*msg).request_number_mut() = reqno;
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_request_number(msg: *mut Msg) -> c_ulonglong {
    assert!(!msg.is_null(), "msg must not be null");
    (*msg).request_number()
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_kind(msg: *mut Msg, kind: *const c_uchar) {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(!kind.is_null(), "kind must not be null");
    let string: String = c_string_to_str(kind).to_owned();
    *(*msg).kind_mut() = string;
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_kind(msg: *mut Msg) -> *const c_uchar {
    assert!(!msg.is_null(), "msg must not be null");
    let kind: &str = (*msg).kind_ref();
    str_to_c_string(kind) as *const c_uchar
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_origin(msg: *mut Msg, origin: *const c_uchar) {
    assert!(!msg.is_null(), "msg must not be null");
    let origin =
        if origin.is_null() { None }
        else { Some(c_string_to_str(origin).to_owned()) };
    *(*msg).origin_mut() = origin;
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_origin(msg: *mut Msg) -> *const c_uchar {
    assert!(!msg.is_null(), "msg must not be null");
    match (*msg).origin_ref() {
        None => std::ptr::null(),
        Some(origin) => str_to_c_string(origin) as *const c_uchar,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_contents(msg: *mut Msg, contents: *mut Contents) {
    assert!(!msg.is_null(), "msg must not be null");
    *(*msg).contents_mut() = consume_raw(contents);
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_contents(msg: *mut Msg) -> *const Contents {
    assert!(!msg.is_null(), "msg must not be null");
    match (*msg).contents_ref() {
        None => std::ptr::null(),
        Some(contents) => contents as *const Contents,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn msg_add_region(msg: *mut Msg, region: *mut Region) {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(!region.is_null(), "region must not be null");
    (*msg).regions_mut().push(*Box::from_raw(region));
}

#[no_mangle]
pub unsafe extern "C"
fn msg_clear_regions(msg: *mut Msg) {
    assert!(!msg.is_null(), "msg must not be null");
    (*msg).regions_mut().clear();
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_region(msg: *mut Msg, index: c_ulonglong)
                                 -> *const Region {
    assert!(!msg.is_null(), "msg must not be null");
    assert!(index < msg_count_regions(msg), "index out of bounds");
    let regions = (*msg).regions_ref();
    &regions[index as usize]
}

#[no_mangle]
pub unsafe extern "C"
fn msg_count_regions(msg: *mut Msg) -> c_ulonglong {
    assert!(!msg.is_null(), "msg must not be null");
    (*msg).regions_ref().len() as c_ulonglong
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_language(msg: *mut Msg, lang: *mut Language) {
    assert!(!msg.is_null(), "msg must not be null");
    *(*msg).language_mut() = consume_raw(lang);
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_language(msg: *mut Msg) -> *const Language {
    assert!(!msg.is_null(), "msg must not be null");
    match (*msg).language_ref() {
        None => std::ptr::null(),
        Some(lang) => lang,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn msg_set_ast(msg: *mut Msg, ast: *mut Ast) {
    assert!(!msg.is_null(), "msg must not be null");
    *(*msg).ast_mut() = consume_raw(ast);
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_ast(msg: *mut Msg) -> *const Ast {
    assert!(!msg.is_null(), "msg must not be null");
    match (*msg).ast_ref() {
        None => std::ptr::null(),
        Some(ast) => ast,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn msg_get_sent_at(msg: *mut Msg) -> c_ulonglong {
    assert!(!msg.is_null(), "msg must not be null");
    (*msg).sent_at()
}

/******************************************************************************/
/*                              Contents                                      */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn contents_new_text(text: *const c_uchar) -> *mut Contents {
    let string: &str = c_string_to_str(text);
    Box::into_raw(Box::new(Contents::from(string)))
}

#[no_mangle]
pub unsafe extern "C"
fn contents_new_entries() -> *mut Contents {
    Box::into_raw(Box::new(Contents::from(vec![])))
}

#[no_mangle]
pub unsafe extern "C"
fn contents_destroy(contents: *mut Contents) {
    assert!(!contents.is_null(), "contents must not be null");
    drop(Box::from_raw(contents))
}

#[no_mangle]
pub unsafe extern "C"
fn contents_is_empty(contents: *mut Contents) -> c_int {
    if contents.is_null() {  return true as c_int;  }
    match &*contents {
        &Contents::Text(ref text) => text.is_empty() as c_int,
        &Contents::Entries(ref entries) => entries.is_empty() as c_int,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn contents_is_text(contents: *mut Contents) -> c_int {
    if contents.is_null() {  return false as c_int;  }
    match &*contents {
        &Contents::Text(_) => true as c_int,
        _ => false as c_int,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn contents_add_text(contents: *mut Contents, text: *const c_uchar) {
    assert!(!contents.is_null(), "contents must not be null");
    assert!(!text.is_null(), "text must not be null");
    match &mut *contents {
        &mut Contents::Text(ref mut t) => t.push_str(c_string_to_str(text)),
        c => panic!("Can only add text to Contents::Text, but found {:?}", c),
    }
}

#[no_mangle]
pub unsafe extern "C"
fn contents_get_text(contents: *mut Contents) -> *const c_uchar {
    assert!(!contents.is_null(), "contents must not be null");
    match &*contents {
        &Contents::Text(ref text) =>
            str_to_c_string(text.as_str()) as *const c_uchar,
        c => panic!("Expected Contents::Text, but found {:?}", c),
    }
}

#[no_mangle]
pub unsafe extern "C"
fn contents_is_entries(contents: *mut Contents) -> c_int {
    if contents.is_null() {  return false as c_int;  }
    match &*contents {
        &Contents::Entries(_) => true as c_int,
        _ => false as c_int,
    }
}

#[no_mangle]
pub unsafe extern "C"
fn contents_add_entry(contents: *mut Contents, entry: *const c_uchar) {
    assert!(!contents.is_null(), "contents must not be null");
    assert!(!entry.is_null(), "entry must not be null");
    let entry: &str = c_string_to_str(entry);
    match &mut *contents {
        &mut Contents::Entries(ref mut vec) => vec.push(String::from(entry)),
        c => panic!("Can only add an entry to Contents::Entries, but found {:?}", c),
    }
}

#[no_mangle]
pub unsafe extern "C"
fn contents_get_entry(contents: *mut Contents, index: c_ulonglong)
                                     -> *const c_uchar {
    assert!(!contents.is_null(), "contents must not be null");
    match &*contents {
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
pub unsafe extern "C"
fn contents_count_entries(contents: *mut Contents) -> c_ulonglong {
    assert!(!contents.is_null(), "contents must not be null");
    match &*contents {
        &Contents::Entries(ref entries) => entries.len() as c_ulonglong,
        c => panic!("Expected Contents::Entries, but found {:?}", c),
    }
}

/******************************************************************************/
/*                              Region                                        */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn region_new(begin: c_ulonglong, end: c_ulonglong) -> *mut Region {
    Box::into_raw(Box::new(Region { begin: begin, end: end }))
}

#[no_mangle]
pub unsafe extern "C"
fn region_destroy(region: *mut Region) {
    assert!(!region.is_null(), "region must not be null");
    drop(Box::from_raw(region))
}

#[no_mangle]
pub unsafe extern "C"
fn region_get_begin(region: *const Region) -> c_ulonglong {
    assert!(!region.is_null(), "region must not be null");
    (*region).begin
}

#[no_mangle]
pub unsafe extern "C"
fn region_get_end(region: *const Region) -> c_ulonglong {
    assert!(!region.is_null(), "region must not be null");
    (*region).end
}

/******************************************************************************/
/*                              Language                                      */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn language_new(language: *const c_uchar) -> *mut Language {
    assert!(!language.is_null(), "language must not be null");
    let string: &str = c_string_to_str(language);
    Box::into_raw(Box::new(Language::from(string)))
}

#[no_mangle]
pub unsafe extern "C"
fn language_destroy(language: *mut Language) {
    assert!(!language.is_null(), "language must not be null");
    drop(Box::from_raw(language))
}

#[no_mangle]
pub unsafe extern "C"
fn language_set_name(language: *mut Language, new: *const c_uchar) {
    assert!(!language.is_null(), "language must not be null");
    assert!(!new.is_null(), "new must not be null");
    let new: &str = c_string_to_str(new);
    *(*language).as_mut() = String::from(new);
}

#[no_mangle]
pub unsafe extern "C"
fn language_get_name(language: *mut Language) -> *const c_char {
    let lang: &str =
        if language.is_null() { "" }
        else { (*language).as_ref() };
    str_to_c_string(lang)
}

/******************************************************************************/
/*                              Ast                                           */
/******************************************************************************/
#[no_mangle]
pub unsafe extern "C"
fn ast_new(name: *const c_uchar) -> *mut Ast {
    let ast = Ast::new(c_string_to_str(name));
    Box::into_raw(Box::new(ast))
}

#[no_mangle]
pub unsafe extern "C"
fn ast_destroy(ast: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    drop(Box::from_raw(ast))
}

#[no_mangle]
pub unsafe extern "C"
fn ast_get_name(ast: *mut Ast) -> *const c_uchar {
    assert!(!ast.is_null(), "ast must not be null");
    let name = (*ast).name_ref();
    str_to_c_string(name) as *const c_uchar
}

#[no_mangle]
pub unsafe extern "C"
fn ast_set_data(ast: *mut Ast, data: *const c_uchar) {
    assert!(!ast.is_null(), "ast must not be null");
    assert!(!data.is_null(), "data must not be null");
    let data: &str = c_string_to_str(data);
    *(*ast).data_mut() = String::from(data);
}

#[no_mangle]
pub unsafe extern "C"
fn ast_get_data(ast: *mut Ast) -> *const c_uchar {
    assert!(!ast.is_null(), "ast must not be null");
    let data = (*ast).data_ref();
    str_to_c_string(data) as *const c_uchar
}

#[no_mangle]
pub unsafe extern "C"
fn ast_clear_data(ast: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    (*ast).data_mut().clear();
}

#[no_mangle]
pub unsafe extern "C"
fn ast_add_child(ast: *mut Ast, child: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    assert!(!child.is_null(), "child must not be null");
    (*ast).children_mut().push(*Box::from_raw(child));
}

#[no_mangle]
pub unsafe extern "C"
fn ast_get_child(ast: *mut Ast, index: c_ulonglong) -> *const Ast {
    assert!(!ast.is_null(), "ast must not be null");
    let num_children = ast_count_children(ast);
    assert!(index < num_children, "index out of bounds: {} >= {}", index, num_children);
    let children = (*ast).children_ref();
    &children[index as usize]
}

#[no_mangle]
pub unsafe extern "C"
fn ast_clear_children(ast: *mut Ast) {
    assert!(!ast.is_null(), "ast must not be null");
    (*ast).children_mut().clear();
}

#[no_mangle]
pub unsafe extern "C"
fn ast_count_children(ast: *mut Ast) -> c_ulonglong {
    assert!(!ast.is_null(), "ast must not be null");
    (*ast).children_ref().len() as c_ulonglong
}

/******************************************************************************/
/*                              utility                                       */
/******************************************************************************/
unsafe fn c_string_to_str<'s>(cstring: *const c_uchar) -> &'s str {
    let cstr = CStr::from_ptr(cstring as *const c_char);
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

        thread::spawn(move || unsafe {
            let sink = uclient_new();
            let sink = uclient_connect(sink);
            let mut msg = Msg::default();

            cclient_receive(sink, &mut msg);
            println!("sink1 received msg");
            io::stdout().flush().unwrap();
            sink1_tx.send(msg).unwrap();
        });

        thread::spawn(move || unsafe {
            let sink = uclient_new();
            let sink = uclient_connect(sink);
            let mut msg = Msg::default();

            cclient_receive(sink, &mut msg);
            println!("sink2 received msg");
            io::stdout().flush().unwrap();
            sink2_tx.send(msg).unwrap();
        });

        thread::spawn(move || unsafe {
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
        thread::spawn(move || unsafe {
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
