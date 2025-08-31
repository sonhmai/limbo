use h3o::{CellIndex, LatLng};
use turso_ext::{Value, ValueType};

/// H3 cell index wrapper for easier conversion
#[derive(Debug, Clone, Copy)]
pub struct H3Cell(pub CellIndex);

impl H3Cell {
    /// Create from raw H3 index value
    pub fn from_raw(raw: u64) -> Result<Self, String> {
        CellIndex::try_from(raw)
            .map(H3Cell)
            .map_err(|e| format!("Invalid H3 index: {}", e))
    }

    /// Get raw H3 index value
    pub fn to_raw(&self) -> u64 {
        self.0.into()
    }

    /// Get resolution of this H3 cell
    pub fn resolution(&self) -> u8 {
        self.0.resolution() as u8
    }

    /// Check if this is a valid H3 cell
    pub fn is_valid(&self) -> bool {
        true // If we can construct it, it's valid
    }
}

/// Convert H3 cell to Turso Value
impl From<H3Cell> for Value {
    fn from(cell: H3Cell) -> Self {
        Value::from_integer(cell.to_raw() as i64)
    }
}

/// Try to convert Turso Value to H3 cell
impl TryFrom<&Value> for H3Cell {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.value_type() {
            ValueType::Integer => {
                let raw = value.to_integer()
                    .ok_or("Failed to extract integer from Value")?;
                H3Cell::from_raw(raw as u64)
            }
            ValueType::Text => {
                let text = value.to_text()
                    .ok_or("Failed to extract text from Value")?;
                let raw = u64::from_str_radix(&text, 16)
                    .map_err(|_| "Invalid hex string for H3 index")?;
                H3Cell::from_raw(raw)
            }
            _ => Err("H3 index must be integer or hex string".to_string())
        }
    }
}

/// Coordinate pair for lat/lng operations
#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

impl Coordinate {
    pub fn new(lat: f64, lng: f64) -> Result<Self, String> {
        if lat < -90.0 || lat > 90.0 {
            return Err("Latitude must be between -90 and 90 degrees".to_string());
        }
        if lng < -180.0 || lng > 180.0 {
            return Err("Longitude must be between -180 and 180 degrees".to_string());
        }
        Ok(Coordinate { lat, lng })
    }

    pub fn to_latlng(&self) -> Result<LatLng, String> {
        LatLng::new(self.lat, self.lng)
            .map_err(|e| format!("Invalid coordinates: {}", e))
    }
}

impl From<LatLng> for Coordinate {
    fn from(latlng: LatLng) -> Self {
        Coordinate {
            lat: latlng.lat(),
            lng: latlng.lng(),
        }
    }
}
