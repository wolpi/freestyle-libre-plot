use chrono::NaiveDateTime;

pub const TIMESTAMP_FORMAT :&str = "%Y.%m.%d %H:%M";

#[derive(Clone)]
pub struct FsLibreLine {
    pub id :String,
    pub timestamp :NaiveDateTime,
    pub line_type :u32,
    pub gluco_hist:u32,
    pub gluco_scanned :u32,
    pub fast_insulin :u32,
    pub fast_insulin_non_numeric :u32,
    pub fast_insulin_units :u32,
    pub food :u32,
    pub food_non_numeric :u32,
    pub carbohydrate :u32,
    pub slow_insulin :u32,
    pub slow_insulin_non_numeric :u32,
    pub slow_insulin_units :u32,
}

impl FsLibreLine {
    pub fn new() -> FsLibreLine {
        FsLibreLine {
            id: String::new(),
            timestamp: NaiveDateTime::parse_from_str("1970.01.01 00:00", TIMESTAMP_FORMAT).unwrap(),
            line_type: 0,
            gluco_hist: 0,
            gluco_scanned: 0,
            fast_insulin: 0,
            fast_insulin_non_numeric: 0,
            fast_insulin_units: 0,
            food: 0,
            food_non_numeric: 0,
            carbohydrate: 0,
            slow_insulin: 0,
            slow_insulin_non_numeric: 0,
            slow_insulin_units: 0,
        }
    }
}
