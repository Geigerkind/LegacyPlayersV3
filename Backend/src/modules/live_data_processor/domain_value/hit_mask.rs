use crate::modules::live_data_processor::domain_value::HitType;

pub type HitMask = Vec<HitType>;

pub fn hit_mask_from_u32(hit_mask: u32) -> HitMask {
    let mut result = Vec::new();
    let mut hit_type: u32 = 1;
    while hit_type < HitType::Reflect as u32 {
        if hit_mask & hit_type > 0 {
            result.push(unsafe { ::std::mem::transmute(hit_type) });
        }
        hit_type *= 2;
    }
    result
}
