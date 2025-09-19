const ONE_VALUE_I64: i64 = 100;
const ONE_VALUE_F64: f64 = 100.0;


pub fn format_int_to_decimal(valor: i64) -> String {
    let part_int = valor / 100;
    let part_float = (valor % 100).abs();
    format!("R$ {},{:02}", part_int, part_float)
}

pub fn validate_float(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    value.replace(",", ".").parse::<f64>().is_ok()
}

pub fn validate_float_range(value: &str, min: f64, max: f64) -> bool {
    if value.is_empty() {
        return true;
    }
    if let Ok(v) = value.replace(",", ".").parse::<f64>() {
        return v >= min && v <= max;
    }
    false
}

pub fn validate_int(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    value.parse::<i32>().is_ok()
}

pub fn f64_to_i64(value: f64) -> i64 {
    (value * ONE_VALUE_F64).round() as i64
}

pub fn i64_to_f64(value: i64) -> f64 {
    (value as f64) / ONE_VALUE_F64
}
