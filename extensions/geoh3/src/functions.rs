use h3o::Resolution;
use turso_ext::{scalar, Value};
use crate::types::{H3Cell, Coordinate};
use crate::utils::*;

/// Convert latitude/longitude coordinates to H3 cell index
/// Usage: h3_from_latlng(lat, lng, resolution)
#[scalar(name = "h3_from_latlng")]
fn h3_from_latlng(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value::from_text("Error: h3_from_latlng requires 3 arguments: lat, lng, resolution".to_string());
    }

    match (|| -> Result<Value, String> {
        let coord = extract_coordinate(args, 0, 1)?;
        let resolution = extract_resolution(args, 2)?;
        
        let latlng = coord.to_latlng()?;
        let res = Resolution::try_from(resolution)
            .map_err(|e| format!("Invalid resolution: {}", e))?;
        
        let cell = latlng.to_cell(res);
        let h3_cell = H3Cell(cell);
        
        Ok(h3_cell.into())
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Convert H3 cell index to center latitude/longitude coordinates
/// Usage: h3_to_latlng(h3_index) -> returns "lat,lng"
#[scalar(name = "h3_to_latlng")]
fn h3_to_latlng(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::from_text("Error: h3_to_latlng requires 1 argument: h3_index".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        let latlng: h3o::LatLng = h3_cell.0.into();
        let coord = Coordinate::from(latlng);
        
        Ok(Value::from_text(format!("{},{}", coord.lat, coord.lng)))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get the boundary of an H3 cell as a polygon
/// Usage: h3_to_boundary(h3_index) -> returns boundary as text
#[scalar(name = "h3_to_boundary")]
fn h3_to_boundary(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::from_text("Error: h3_to_boundary requires 1 argument: h3_index".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        let boundary = h3_cell.0.boundary();
        
        let coords: Vec<String> = boundary.iter()
            .map(|latlng| format!("{},{}", latlng.lat(), latlng.lng()))
            .collect();
        
        Ok(Value::from_text(format!("[{}]", coords.join(";"))))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get the neighboring cells of an H3 cell
/// Usage: h3_neighbors(h3_index) -> returns neighbors as text array
#[scalar(name = "h3_neighbors")]
fn h3_neighbors(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::from_text("Error: h3_neighbors requires 1 argument: h3_index".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        let neighbors: Vec<H3Cell> = h3_cell.0.grid_ring_fast(1)
            .filter_map(|opt| opt)
            .map(H3Cell)
            .collect();
        
        Ok(Value::from_text(cells_to_text(neighbors)))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get the parent cell at a coarser resolution
/// Usage: h3_parent(h3_index, parent_resolution)
#[scalar(name = "h3_parent")]
fn h3_parent(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::from_text("Error: h3_parent requires 2 arguments: h3_index, parent_resolution".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        let parent_res = extract_resolution(args, 1)?;
        
        let resolution = Resolution::try_from(parent_res)
            .map_err(|e| format!("Invalid parent resolution: {}", e))?;
        
        if parent_res >= h3_cell.resolution() {
            return Err("Parent resolution must be coarser (smaller) than child resolution".to_string());
        }
        
        let parent = h3_cell.0.parent(resolution)
            .ok_or("Failed to get parent cell")?;
        let parent_cell = H3Cell(parent);
        
        Ok(parent_cell.into())
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get the child cells at a finer resolution
/// Usage: h3_children(h3_index, child_resolution) -> returns children as text array
#[scalar(name = "h3_children")]
fn h3_children(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::from_text("Error: h3_children requires 2 arguments: h3_index, child_resolution".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        let child_res = extract_resolution(args, 1)?;
        
        let resolution = Resolution::try_from(child_res)
            .map_err(|e| format!("Invalid child resolution: {}", e))?;
        
        if child_res <= h3_cell.resolution() {
            return Err("Child resolution must be finer (larger) than parent resolution".to_string());
        }
        
        let children: Vec<H3Cell> = h3_cell.0.children(resolution)
            .map(H3Cell)
            .collect();
        
        Ok(Value::from_text(cells_to_text(children)))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get the grid distance between two H3 cells
/// Usage: h3_distance(h3_index1, h3_index2)
#[scalar(name = "h3_distance")]
fn h3_distance(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::from_text("Error: h3_distance requires 2 arguments: h3_index1, h3_index2".to_string());
    }

    match (|| -> Result<Value, String> {
        let cell1 = extract_h3_cell(args, 0)?;
        let cell2 = extract_h3_cell(args, 1)?;
        
        if cell1.resolution() != cell2.resolution() {
            return Err("Both cells must be at the same resolution".to_string());
        }
        
        let distance = cell1.0.grid_distance(cell2.0)
            .map_err(|e| format!("Failed to calculate distance: {}", e))?;
        
        Ok(Value::from_integer(distance as i64))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get all cells within k distance of an H3 cell
/// Usage: h3_k_ring(h3_index, k) -> returns cells as text array
#[scalar(name = "h3_k_ring")]
fn h3_k_ring(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::from_text("Error: h3_k_ring requires 2 arguments: h3_index, k".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        let k = extract_integer(args, 1)?;
        
        if k < 0 {
            return Err("k must be non-negative".to_string());
        }
        
        if k > 100 {
            return Err("k must be <= 100 to prevent excessive memory usage".to_string());
        }
        
        let cells: Vec<H3Cell> = h3_cell.0.grid_disk_fast(k as u32)
            .filter_map(|opt| opt)
            .map(H3Cell)
            .collect();
        
        Ok(Value::from_text(cells_to_text(cells)))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Get the resolution of an H3 cell
/// Usage: h3_resolution(h3_index)
#[scalar(name = "h3_resolution")]
fn h3_resolution(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::from_text("Error: h3_resolution requires 1 argument: h3_index".to_string());
    }

    match (|| -> Result<Value, String> {
        let h3_cell = extract_h3_cell(args, 0)?;
        Ok(Value::from_integer(h3_cell.resolution() as i64))
    })() {
        Ok(value) => value,
        Err(err) => Value::from_text(format!("Error: {}", err))
    }
}

/// Check if an H3 index is valid
/// Usage: h3_is_valid(h3_index) -> returns 1 for valid, 0 for invalid
#[scalar(name = "h3_is_valid")]
fn h3_is_valid(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::from_integer(0);
    }

    match H3Cell::try_from(&args[0]) {
        Ok(_) => Value::from_integer(1),
        Err(_) => Value::from_integer(0)
    }
}
