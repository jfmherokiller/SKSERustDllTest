use commonlibsse_sys::root::SKSE;
lazy_static! {
    static ref SUPPORTED_VERSION: Vec<u32> = vec![0x01050610u32];
}

#[no_mangle]
pub extern fn lib_test() {
    println!("Hello from the library!");
}
#[no_mangle]
pub extern fn SKSEPlugin_Query(a_skse:* const SKSE::QueryInterface,a_info:* const SKSE::PluginInfo) -> bool {
    let askse = unsafe { a_skse.as_ref() }.unwrap();
    unsafe {
        if askse.IsEditor() {
            return false;
        }
        let ver = askse.RuntimeVersion();
    }
    return true;
}

#[no_mangle]
pub extern fn SKSEPlugin_Load(pluginter:* const SKSE::LoadInterface) -> bool {
    let askse = unsafe { pluginter.as_ref() }.unwrap();
    unsafe {
        SKSE::Init(askse);
    }
    return true;
}