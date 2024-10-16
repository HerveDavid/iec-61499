use libloading::{Library, Symbol};
use std::ffi::{c_char, c_int, c_void, CStr, CString};

use graal_types::{graal_create_isolate_params_t, graal_isolate_t, graal_isolatethread_t};

mod graal_types;

type GraalCreateIsolate = extern "C" fn(
    params: *mut graal_create_isolate_params_t,
    isolate: *mut *mut graal_isolate_t,
    thread: *mut *mut graal_isolatethread_t,
) -> c_int;

type CreateComponent = extern "C" fn(*mut graal_isolatethread_t, *const c_char) -> *mut c_void;

type GetProcess = extern "C" fn(*mut graal_isolatethread_t, *mut c_void) -> *mut c_void;

type RunProcess = extern "C" fn(*mut graal_isolatethread_t, *mut c_void, c_int) -> *const c_char;

type GetProcessSize = extern "C" fn(*mut graal_isolatethread_t, *mut c_void) -> c_int;

type DisposeHandle = extern "C" fn(*mut graal_isolatethread_t, *mut c_void) -> c_void;

fn main() {
    let lib = unsafe {
        Library::new("/home/david/Projets/IEC-61499/runtime-node/libenv/target/libenv.so").unwrap()
    };

    // Functions
    let create_component: Symbol<CreateComponent> =
        unsafe { lib.get(b"create_component").unwrap() };

    let get_process: Symbol<GetProcess> = unsafe { lib.get(b"get_process").unwrap() };

    let run_process: Symbol<RunProcess> = unsafe { lib.get(b"run_process").unwrap() };

    let get_process_size: Symbol<GetProcessSize> =
        unsafe { lib.get(b"get_processes_size").unwrap() };

    let dispose_handle: Symbol<DisposeHandle> = unsafe { lib.get(b"dispose_handle").unwrap() };

    let graal_create_isolate: Symbol<GraalCreateIsolate> =
        unsafe { lib.get(b"graal_create_isolate").unwrap() };

    let mut isolate_params = graal_create_isolate_params_t::new();
    let mut isolate: *mut graal_isolate_t = std::ptr::null_mut();
    let mut thread: *mut graal_isolatethread_t = std::ptr::null_mut();
    graal_create_isolate(&mut isolate_params, &mut isolate, &mut thread);

    let name = CString::new("cpt").unwrap();
    let component = create_component(thread, name.as_ptr());
    let processes = get_process(thread, component);
    let size = get_process_size(thread, processes);

    for i in 0..size {
        let result = run_process(thread, processes, i);
        let result = unsafe { CStr::from_ptr(result).to_str().unwrap() };
        println!("Result: {}", result);
    }

    dispose_handle(thread, component);
    dispose_handle(thread, processes);
}
