use openconstruct_abi::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn cstr(s: &str) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

#[test]
fn test_registry_create_has_modules() {
    let reg = oc_registry_create();
    assert!(!reg.is_null());
    let count = oc_registry_count(reg);
    assert_eq!(count, 10);
    oc_registry_free(reg);
}

#[test]
fn test_registry_module_iteration() {
    let reg = oc_registry_create();
    let count = oc_registry_count(reg);
    for i in 0..count {
        let m = oc_registry_get(reg, i);
        assert!(!m.is_null());
        unsafe {
            let id = CStr::from_ptr((*m).id).to_str().unwrap();
            assert!(!id.is_empty());
        }
    }
    oc_registry_free(reg);
}

#[test]
fn test_session_full_lifecycle() {
    let session = oc_session_start();
    assert_eq!(oc_session_phase(session), OC_PHASE_SELF_DECLARATION);

    let r = oc_declare_agent(session, cstr("TestBot"), cstr("llama-3"));
    assert!(r.success);
    assert_eq!(oc_session_phase(session), OC_PHASE_MODULE_SELECTION);
    oc_string_free(r.message);
    oc_string_free(r.data_json);

    let r = oc_select_modules(session, cstr("[\"spectral-graph-core\"]"));
    assert!(r.success);
    oc_string_free(r.message);
    oc_string_free(r.data_json);

    let config = oc_generate_config(session);
    let config_str = unsafe { CStr::from_ptr(config).to_str().unwrap() };
    assert!(config_str.contains("TestBot"));
    assert!(config_str.contains("spectral-graph-core"));
    oc_string_free(config);
    oc_session_free(session);
}

#[test]
fn test_null_safety() {
    assert_eq!(oc_registry_count(std::ptr::null()), 0);
    assert!(oc_registry_get(std::ptr::null(), 0).is_null());
    assert_eq!(oc_session_phase(std::ptr::null()), 0);
}

#[test]
fn test_registry_get_out_of_bounds() {
    let reg = oc_registry_create();
    assert!(oc_registry_get(reg, 999).is_null());
    oc_registry_free(reg);
}
