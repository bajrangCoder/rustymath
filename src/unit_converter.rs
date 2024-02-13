pub struct Unit {
    pub name: &'static str,
    pub conversion_factor: f64,
}

impl Unit {
    pub fn convert(&self, value: f64, to_unit: &Unit) -> f64 {
        value * (self.conversion_factor / to_unit.conversion_factor)
    }
}

pub fn convert_unit(value: f64, from_unit: &Unit, to_unit: &Unit) -> f64 {
    from_unit.convert(value, to_unit)
}
