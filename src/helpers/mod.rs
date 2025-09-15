pub fn format_int_to_decimal(valor: i32) -> String {
    let part_int = valor / 100;
    let part_float = (valor % 100).abs();
    format!("R$ {},{:02}", part_int, part_float)
}

pub fn validate_float(value: &str) -> bool {
    value.replace(",", ".").parse::<f64>().is_ok()
}

pub fn validate_int(value: &str) -> bool {
    value.parse::<i32>().is_ok()
}
