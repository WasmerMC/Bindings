extern "C" {
    fn platform_set_modname(len1: i32, off1: i32, len2: i32, off2: i32);
}

pub fn set_modname(mod_id: &str, mod_name: &str) {
    let id_len = mod_id.len() as i32;
    let id_off = mod_id.as_ptr() as i32;

    let dt_len = mod_name.len() as i32;
    let dt_off = mod_name.as_ptr() as i32;

    unsafe {
        platform_set_modname(id_len, id_off, dt_len, dt_off);
    }
}