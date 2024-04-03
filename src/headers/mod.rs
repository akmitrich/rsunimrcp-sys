mod synth;
pub use synth::SynthHeaders;

mod recog;
pub use recog::RecogHeaders;

use crate::{inline_mrcp_generic_header_get, inline_mrcp_generic_header_property_check, uni};
use std::collections::HashMap;

fn extract_vendor_specific_parameters(
    request: *const uni::mrcp_message_t,
) -> HashMap<String, String> {
    if request.is_null() {
        return HashMap::new();
    }
    let mut params = HashMap::new();
    unsafe {
        if inline_mrcp_generic_header_property_check(
            request,
            uni::GENERIC_HEADER_VENDOR_SPECIFIC_PARAMS as _,
        ) == uni::TRUE
        {
            let h = inline_mrcp_generic_header_get(request);
            if !h.is_null() {
                let vendor_parameters = (*h).vendor_specific_params;
                let pairs = (*vendor_parameters).elts as *mut uni::apt_pair_t;
                for i in 0..(*vendor_parameters).nelts {
                    let pair = pairs.offset(i as _) as *mut uni::apt_str_t;
                    let key = &*pair.offset(0);
                    let value = &*pair.offset(1);
                    if let (Some(key), Some(value)) =
                        (apt_str_to_string(key), apt_str_to_string(value))
                    {
                        params.insert(key, value);
                    };
                }
            }
        }
    }
    params
}

fn apt_str_to_string(origin: &uni::apt_str_t) -> Option<String> {
    unsafe {
        let ptr = origin.buf as *const u8;
        let len = origin.length;
        if len == 0 || ptr.is_null() {
            None
        } else {
            let as_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8(as_slice).ok().map(|s| s.to_owned())
        }
    }
}
