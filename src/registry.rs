use crate::item::ItemSettings;

extern "C" {
    fn registry_register(bt: i32, len1: i32, off1: i32, len2: i32, off2: i32);
}

pub enum Registries {
    BLOCK,
    ITEM
}

pub fn register(registry_type: Registries, identifier: String, item_settings: ItemSettings) -> Result<(), serde_json::Error>  {
    match registry_type {
        Registries::BLOCK => {
            // block
        }
        Registries::ITEM => {
            // item

            let data = serde_json::to_string(&item_settings)?;

            let id_len = identifier.len() as i32;
            let id_off = identifier.as_ptr() as i32;

            let dt_len = data.len() as i32;
            let dt_off = data.as_ptr() as i32;

            unsafe {
                registry_register(1, id_len, id_off, dt_len, dt_off);
            }
        }
    }
    
    Ok(())
}