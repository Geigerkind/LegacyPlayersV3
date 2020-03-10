// Check if this breakdown is effectively null, i.e. 000|000|000
pub fn strip_talent_specialization(spec: &Option<String>) -> Option<String> {
    let mut talent_specialization: Option<String> = None;
    if spec.is_some() {
        if spec
            .as_ref()
            .unwrap()
            .split('|')
            .map(|spec| {
                spec.chars()
                    .map(|talent| talent.to_digit(10).unwrap())
                    .sum::<u32>()
            })
            .sum::<u32>()
            > 0
        {
            talent_specialization = spec.clone();
        }
    }
    return talent_specialization;
}
