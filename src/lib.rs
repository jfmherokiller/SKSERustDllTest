use commonlibsse_sys::root::SKSE;


#[no_mangle]
pub extern fn lib_test() {
    println!("Hello from the library!");
}
#[no_mangle]
pub extern fn SKSEPlugin_Query(a_skse:* const SKSE::QueryInterface,a_info:* const SKSE::PluginInfo) -> bool {
    return query_body(a_skse,a_info);
}


#[no_mangle]
pub extern fn SKSEPlugin_Load(pluginter:* const SKSE::LoadInterface) -> bool {
    return plugin_load_body(pluginter);
}

fn plugin_load_body(pluginter:* const SKSE::LoadInterface) -> bool {
    let askse = unsafe { pluginter.as_ref() }.unwrap();
    unsafe {
        SKSE::Init(askse);
    }
    return true;
}
fn query_body(a_skse:* const SKSE::QueryInterface,a_info:* const SKSE::PluginInfo) -> bool{
    let askse = unsafe { a_skse.as_ref() }.unwrap();
    unsafe {
        if askse.IsEditor() {
            return false;
        }
        let ver = askse.RuntimeVersion();
    }
    return true;
}