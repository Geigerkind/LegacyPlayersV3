use crate::modules::live_data_processor::domain_value::School;

pub type SchoolMask = Vec<School>;

pub fn school_mask_from_u8(school_mask: u8) -> SchoolMask {
    let mut result = Vec::new();
    let mut school: u8 = 1;
    while school < School::Arcane as u8 {
        if school_mask & school > 0 {
            result.push(unsafe { ::std::mem::transmute(school) });
        }
        school *= 2;
    }
    result
}
