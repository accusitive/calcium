#![feature(set_ptr_value)]

use std::{ffi::CString, mem::MaybeUninit};
#[allow(
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    non_camel_case_types,
    deref_nullptr,
    improper_ctypes
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let mut scanner: MaybeUninit<bindings::yyscan_t> = MaybeUninit::uninit();
    // let buf: bindings::YY_BUFFER_STATE;
    let st = CString::new("hello world").unwrap();
    unsafe {
        let s = st.as_ptr() as *const i8;

        bindings::yylex_init(scanner.as_mut_ptr());

        let buf = bindings::yy_scan_string(s, scanner.assume_init());
        bindings::yy_switch_to_buffer(buf, scanner.assume_init());
        bindings::yylex(scanner.assume_init());
        bindings::yy_delete_buffer(buf, scanner.assume_init());
        bindings::yylex_destroy(scanner.assume_init());
    }
    // unsafe {
    //     let s=  "a test string";
    //     let cs = CString::new(s).unwrap();

    //     bindings::yy_scan_string(cs.as_ptr() as *mut i8);
    //     bindings::yylex();
    // }
}
#[no_mangle]
extern "C" fn yywrap() -> i32 {
    return 0;
}

// const globalInputText: &[u8] = b"Test";
// static mut globalReadOffset: usize = 0;

// #[no_mangle]
// extern "C" fn readInputForLexer(
//     mut buffer: &mut [i8],
//     numBytesRead: *mut i32,
//     maxBytesToRead: i32,
// ) -> i32 {
//     unsafe {
//         let mut numBytesToRead = maxBytesToRead as usize;
//         let bytes_remaining = globalInputText.len() - globalReadOffset as usize;
//         let mut i;
//         if numBytesToRead > bytes_remaining {
//             numBytesToRead = bytes_remaining;
//         }
//         for j in 0..numBytesToRead {
//             buffer[j] = *globalInputText.get(globalReadOffset+j).unwrap() as i8;
//             i = j;
//         }
//         numBytesRead.write(numBytesToRead.try_into().unwrap());
//         globalReadOffset += numBytesToRead;
//         }

//     0
// }
