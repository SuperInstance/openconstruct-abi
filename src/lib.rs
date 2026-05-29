//! OpenConstruct C ABI — the keystone for polyglot bindings.
//! Any language that can call C functions can use OpenConstruct.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Opaque session handle
pub struct OcSession {
    pub id: String,
    pub phase: u8,
    pub agent_name: String,
    pub agent_model: String,
    pub selected_modules: Vec<String>,
}

/// Opaque registry handle
pub struct OcRegistry {
    pub modules: Vec<OcModule>,
}

/// Module descriptor
#[repr(C)]
pub struct OcModule {
    pub id: *mut c_char,
    pub domain: *mut c_char,
    pub name: *mut c_char,
    pub one_line: *mut c_char,
}

/// Onboarding result
#[repr(C)]
pub struct OcResult {
    pub success: bool,
    pub phase: u8,
    pub message: *mut c_char,
    pub data_json: *mut c_char,
}

/// Phase constants
pub const OC_PHASE_SELF_DECLARATION: u8 = 1;
pub const OC_PHASE_MODULE_SELECTION: u8 = 2;
pub const OC_PHASE_INTERFACE_SELECTION: u8 = 3;
pub const OC_PHASE_CONNECTION_SETUP: u8 = 4;
pub const OC_PHASE_ENVIRONMENT_GEN: u8 = 5;

// ===== C API Functions =====

/// Create a new OpenConstruct registry with default SuperInstance modules
#[unsafe(no_mangle)]
pub extern "C" fn oc_registry_create() -> *mut OcRegistry {
    let modules = vec![
        make_module("spectral-graph-core", "math", "Spectral Graph Core", "Laplacian eigenvalues, conservation ratio, Fiedler analysis"),
        make_module("conservation-regime", "math", "Conservation Regime", "CR regime detection, anomaly analysis, spectral forecasting"),
        make_module("sheaf-cohomology", "math", "Sheaf Cohomology", "Cellular sheaves, cohomology, consistency on graphs"),
        make_module("symplectic-geometry", "math", "Symplectic Geometry", "Hamiltonian systems, symplectic integrators, conservation"),
        make_module("plato-room", "plato", "PLATO Room", "Knowledge rooms, spectral graphs, tile dependencies"),
        make_module("plato-puppeteer", "plato", "PLATO Puppeteer", "Desktop-to-MUD translation, UI navigation as text"),
        make_module("plato-manus", "plato", "PLATO Manus", "File ops, API calls, device control as text commands"),
        make_module("conservation-protocol", "agents", "Conservation Protocol", "Agent-to-agent via Laplacians, eigenvalues ARE the message"),
        make_module("spectral-deadband", "math", "Spectral Deadband", "Deadband = spectral gap, spin = time, fractal CR"),
        make_module("tropical-algebra", "math", "Tropical Algebra", "Max-plus semiring, tropical polynomials, ReLU equivalence"),
    ];
    Box::into_raw(Box::new(OcRegistry { modules }))
}

/// Free a registry
#[unsafe(no_mangle)]
pub extern "C" fn oc_registry_free(reg: *mut OcRegistry) {
    if !reg.is_null() {
        unsafe {
            for m in &(*reg).modules {
                if !m.id.is_null() { drop(CString::from_raw(m.id)); }
                if !m.domain.is_null() { drop(CString::from_raw(m.domain)); }
                if !m.name.is_null() { drop(CString::from_raw(m.name)); }
                if !m.one_line.is_null() { drop(CString::from_raw(m.one_line)); }
            }
            drop(Box::from_raw(reg));
        }
    }
}

/// Get module count
#[unsafe(no_mangle)]
pub extern "C" fn oc_registry_count(reg: *const OcRegistry) -> usize {
    if reg.is_null() { return 0; }
    unsafe { (*reg).modules.len() }
}

/// Get module by index (returns pointer into registry, do not free)
#[unsafe(no_mangle)]
pub extern "C" fn oc_registry_get(reg: *const OcRegistry, index: usize) -> *const OcModule {
    if reg.is_null() { return ptr::null(); }
    unsafe { (*reg).modules.as_slice().get(index).map(|m| m as *const _).unwrap_or(ptr::null()) }
}

/// Start a new onboarding session
#[unsafe(no_mangle)]
pub extern "C" fn oc_session_start() -> *mut OcSession {
    Box::into_raw(Box::new(OcSession {
        id: format!("oc-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()),
        phase: OC_PHASE_SELF_DECLARATION,
        agent_name: String::new(),
        agent_model: String::new(),
        selected_modules: Vec::new(),
    }))
}

/// Free a session
#[unsafe(no_mangle)]
pub extern "C" fn oc_session_free(session: *mut OcSession) {
    if !session.is_null() { unsafe { drop(Box::from_raw(session)); } }
}

/// Phase 1: Declare agent identity
#[unsafe(no_mangle)]
pub extern "C" fn oc_declare_agent(
    session: *mut OcSession,
    name: *const c_char,
    model: *const c_char,
) -> OcResult {
    if session.is_null() {
        return make_result(false, OC_PHASE_SELF_DECLARATION, "Null session", "{}");
    }
    unsafe {
        (*session).agent_name = cstr_to_string(name);
        (*session).agent_model = cstr_to_string(model);
        (*session).phase = OC_PHASE_MODULE_SELECTION;
    }
    make_result(true, OC_PHASE_MODULE_SELECTION, "Agent declared", "{}")
}

/// Phase 2: Select modules
#[unsafe(no_mangle)]
pub extern "C" fn oc_select_modules(
    session: *mut OcSession,
    module_ids_json: *const c_char,
) -> OcResult {
    if session.is_null() {
        return make_result(false, OC_PHASE_MODULE_SELECTION, "Null session", "{}");
    }
    let ids_str = cstr_to_string(module_ids_json);
    // Simple JSON array parse: ["a", "b"]
    let ids: Vec<String> = ids_str
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect();

    unsafe {
        (*session).selected_modules = ids;
        (*session).phase = OC_PHASE_INTERFACE_SELECTION;
    }
    make_result(true, OC_PHASE_INTERFACE_SELECTION, "Modules selected", &ids_str)
}

/// Get current phase
#[unsafe(no_mangle)]
pub extern "C" fn oc_session_phase(session: *const OcSession) -> u8 {
    if session.is_null() { return 0; }
    unsafe { (*session).phase }
}

/// Generate final config (Phase 5)
#[unsafe(no_mangle)]
pub extern "C" fn oc_generate_config(session: *const OcSession) -> *mut c_char {
    if session.is_null() { return cstr_to_ptr("null session"); }
    let config = unsafe {
        format!(
            "{{\"session_id\": \"{}\", \"agent\": \"{}\", \"model\": \"{}\", \"modules\": {:?}, \"interfaces\": [\"cli\", \"api\"], \"phase\": {}}}",
            (*session).id, (*session).agent_name, (*session).agent_model,
            (*session).selected_modules, (*session).phase
        )
    };
    cstr_to_ptr(&config)
}

/// Free a string returned by the API
#[unsafe(no_mangle)]
pub extern "C" fn oc_string_free(s: *mut c_char) {
    if !s.is_null() { unsafe { drop(CString::from_raw(s)); } }
}

/// Free an OcResult's strings
#[unsafe(no_mangle)]
pub extern "C" fn oc_result_free(result: &mut OcResult) {
    if !result.message.is_null() {
        unsafe { drop(CString::from_raw(result.message)); }
        result.message = ptr::null_mut();
    }
    if !result.data_json.is_null() {
        unsafe { drop(CString::from_raw(result.data_json)); }
        result.data_json = ptr::null_mut();
    }
}

// ===== Helpers =====

fn make_module(id: &str, domain: &str, name: &str, one_line: &str) -> OcModule {
    OcModule {
        id: cstr_to_ptr(id),
        domain: cstr_to_ptr(domain),
        name: cstr_to_ptr(name),
        one_line: cstr_to_ptr(one_line),
    }
}

fn make_result(success: bool, phase: u8, message: &str, data_json: &str) -> OcResult {
    OcResult {
        success,
        phase,
        message: cstr_to_ptr(message),
        data_json: cstr_to_ptr(data_json),
    }
}

fn cstr_to_ptr(s: &str) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() { return String::new(); }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_create_free() {
        let reg = oc_registry_create();
        assert!(!reg.is_null());
        let count = oc_registry_count(reg);
        assert_eq!(count, 10);
        oc_registry_free(reg);
    }

    #[test]
    fn registry_get_module() {
        let reg = oc_registry_create();
        let m = oc_registry_get(reg, 0);
        assert!(!m.is_null());
        unsafe {
            let id = CStr::from_ptr((*m).id).to_str().unwrap();
            assert_eq!(id, "spectral-graph-core");
        }
        oc_registry_free(reg);
    }

    #[test]
    fn session_lifecycle() {
        let session = oc_session_start();
        assert!(!session.is_null());
        assert_eq!(oc_session_phase(session), OC_PHASE_SELF_DECLARATION);

        let result = oc_declare_agent(session, cstr_to_ptr("TestAgent"), cstr_to_ptr("gpt-4"));
        assert!(result.success);
        assert_eq!(oc_session_phase(session), OC_PHASE_MODULE_SELECTION);
        let _ = result; // result fields freed individually
        assert_eq!(oc_session_phase(session), OC_PHASE_MODULE_SELECTION);

        let result = oc_select_modules(session, cstr_to_ptr("[\"spectral-graph-core\",\"plato-room\"]"));
        assert!(result.success);
        assert_eq!(oc_session_phase(session), OC_PHASE_INTERFACE_SELECTION);
        oc_string_free(result.message);
        oc_string_free(result.data_json);

        let config_ptr = oc_generate_config(session);
        let config = unsafe { CStr::from_ptr(config_ptr).to_str().unwrap().to_string() };
        assert!(config.contains("spectral-graph-core"));
        assert!(config.contains("plato-room"));
        oc_string_free(config_ptr);

        oc_session_free(session);
    }

    #[test]
    fn null_safety() {
        assert_eq!(oc_registry_count(std::ptr::null()), 0);
        assert!(oc_registry_get(std::ptr::null(), 0).is_null());
        assert_eq!(oc_session_phase(std::ptr::null()), 0);
    }

    #[test]
    fn module_iteration() {
        let reg = oc_registry_create();
        let count = oc_registry_count(reg);
        let mut domains = Vec::new();
        for i in 0..count {
            let m = oc_registry_get(reg, i);
            assert!(!m.is_null());
            unsafe {
                domains.push(CStr::from_ptr((*m).domain).to_str().unwrap().to_string());
            }
        }
        assert!(domains.contains(&"math".to_string()));
        assert!(domains.contains(&"plato".to_string()));
        assert!(domains.contains(&"agents".to_string()));
        oc_registry_free(reg);
    }

    #[test]
    fn config_generation() {
        let session = oc_session_start();
        oc_declare_agent(session, cstr_to_ptr("Claude"), cstr_to_ptr("claude-4"));
        oc_select_modules(session, cstr_to_ptr("[\"plato-puppeteer\"]"));
        let config_ptr = oc_generate_config(session);
        let config = unsafe { CStr::from_ptr(config_ptr).to_str().unwrap().to_string() };
        assert!(config.contains("Claude"));
        assert!(config.contains("plato-puppeteer"));
        oc_string_free(config_ptr);
        oc_session_free(session);
    }
}
