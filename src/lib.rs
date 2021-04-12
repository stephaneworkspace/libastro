extern crate libc;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
//#[macro_use]
extern crate strum_macros;
pub use astrology::svg_draw;
pub use astrology::svg_draw::DataChartNatal;
pub use astrology::svg_draw::{DataObjectSvg, DataObjectType};
pub use libc::size_t;
use libswe_sys::sweconst::Language;
use libswe_sys::swerust::handler_swe02;
use num_traits::FromPrimitive;
use std::ffi::{CStr, CString};
pub use std::os::raw::{c_char, c_double, c_int};

/// Return version of api
#[no_mangle]
pub extern "C" fn sweversion() -> *const c_char {
    //CString::new(get_version()).unwrap().into_raw()
    CString::new(handler_swe02::version()).unwrap().into_raw()
}

/// Compute natal chart in json
///
/// params :
/// year     -> year of birth
/// month    -> month of birth
/// day      -> day of birth
/// hour     -> hour of birth
/// min      -> minute of birth
/// sec      -> second of birth
/// lat      -> latitude of birth
/// lng      -> longitude of birth
/// max_size -> size in pixel
/// language -> 0 = english / 1 = french
#[no_mangle]
pub extern "C" fn compute(
    year: c_int,
    month: c_int,
    day: c_int,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    max_size: c_double,
    language: c_int,
    path: *const c_char,
) -> *const c_char {
    let lang: Language = match FromPrimitive::from_i32(language as i32) {
        Some(Language::English) => Language::English,
        Some(Language::French) => Language::French,
        None => Language::English,
    };
    let d = DataChartNatal {
        year: year,
        month: month,
        day: day,
        hour: hour,
        min: min,
        sec: sec as f32,
        lat: lat as f32,
        lng: lng as f32,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = svg_draw::chart(max_size as f32, d, &path_str, lang);
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

/// Compute natal with transit chart in json
///
/// params :
/// year          -> year of birth
/// month         -> month of birth
/// day           -> day of birth
/// hour          -> hour of birth
/// min           -> minute of birth
/// sec           -> second of birth
/// lat           -> latitude of birth
/// lng           -> longitude of birth
/// year_transit  -> year of birth
/// month_transit -> month of birth
/// day_transit   -> day of birth
/// hour_transit  -> hour of birth
/// min_transit   -> minute of birth
/// seco_transit  -> second of birth
/// lat_transit   -> latitude of birth
/// lng_transit   -> longitude of birth
/// max_size      -> size in pixel
/// language      -> 0 = english / 1 = french
#[no_mangle]
pub extern "C" fn compute_transit(
    year: c_int,
    month: c_int,
    day: c_int,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    year_transit: c_int,
    month_transit: c_int,
    day_transit: c_int,
    hour_transit: c_int,
    min_transit: c_int,
    sec_transit: c_double,
    lat_transit: c_double,
    lng_transit: c_double,
    max_size: c_double,
    language: c_int,
    path: *const c_char,
) -> *const c_char {
    let lang: Language = match FromPrimitive::from_i32(language as i32) {
        Some(Language::English) => Language::English,
        Some(Language::French) => Language::French,
        None => Language::English,
    };
    let d = DataChartNatal {
        year: year,
        month: month,
        day: day,
        hour: hour,
        min: min,
        sec: sec as f32,
        lat: lat as f32,
        lng: lng as f32,
    };
    let d_t = DataChartNatal {
        year: year_transit,
        month: month_transit,
        day: day_transit,
        hour: hour_transit,
        min: min_transit,
        sec: sec_transit as f32,
        lat: lat_transit as f32,
        lng: lng_transit as f32,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data =
        svg_draw::chart_with_transit(max_size as f32, d, d_t, &path_str, lang);
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

/// Return aspect json
///
/// params :
/// language -> 0 = english / 1 = french
#[no_mangle]
pub extern "C" fn aspects(language: c_int) -> *const c_char {
    let lang: Language = match FromPrimitive::from_i32(language as i32) {
        Some(Language::English) => Language::English,
        Some(Language::French) => Language::French,
        None => Language::English,
    };
    let data = svg_draw::all_aspects(lang);
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

/// Unit test
#[cfg(test)]
mod tests {
    use libswe_sys::swerust::handler_swe02;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn version() {
        assert_eq!(handler_swe02::version(), "2.08");
    }
}
