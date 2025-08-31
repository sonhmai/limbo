use turso_ext::Value;
use crate::types::{H3Cell, Coordinate};

/// Extract latitude from function arguments
pub fn extract_lat(args: &[Value], index: usize) -> Result<f64, String> {
    args.get(index)
        .and_then(|v| v.to_float())
        .ok_or_else(|| format!("Missing or invalid latitude at argument {}", index))
}

/// Extract longitude from function arguments
pub fn extract_lng(args: &[Value], index: usize) -> Result<f64, String> {
    args.get(index)
        .and_then(|v| v.to_float())
        .ok_or_else(|| format!("Missing or invalid longitude at argument {}", index))
}

/// Extract resolution from function arguments
pub fn extract_resolution(args: &[Value], index: usize) -> Result<u8, String> {
    let res = args.get(index)
        .and_then(|v| v.to_integer())
        .ok_or_else(|| format!("Missing or invalid resolution at argument {}", index))?;
    
    if res < 0 || res > 15 {
        return Err("H3 resolution must be between 0 and 15".to_string());
    }
    
    Ok(res as u8)
}

/// Extract H3 cell from function arguments
pub fn extract_h3_cell(args: &[Value], index: usize) -> Result<H3Cell, String> {
    args.get(index)
        .ok_or_else(|| format!("Missing H3 cell at argument {}", index))
        .and_then(H3Cell::try_from)
}

/// Extract integer from function arguments
pub fn extract_integer(args: &[Value], index: usize) -> Result<i64, String> {
    args.get(index)
        .and_then(|v| v.to_integer())
        .ok_or_else(|| format!("Missing or invalid integer at argument {}", index))
}

/// Create coordinate from lat/lng arguments
pub fn extract_coordinate(args: &[Value], lat_idx: usize, lng_idx: usize) -> Result<Coordinate, String> {
    let lat = extract_lat(args, lat_idx)?;
    let lng = extract_lng(args, lng_idx)?;
    Coordinate::new(lat, lng)
}

/// Convert vector of H3 cells to JSON-like text representation
pub fn cells_to_text(cells: Vec<H3Cell>) -> String {
    let hex_strings: Vec<String> = cells.iter()
        .map(|cell| format!("{:x}", cell.to_raw()))
        .collect();
    format!("[{}]", hex_strings.join(","))
}
