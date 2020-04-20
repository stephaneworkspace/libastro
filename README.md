# Traditional astrology for rust

Rust library by Stéphane Bressani (s.bressani@bluewin.ch)

Using swissephem c library by Astrodienst AG
by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an CC License, which allows it to be used 
also on commercial projects. This software uses the swiss ephemeris which is
licensed GPL.

Therefore, if you want to use astro_compute_swisseph in your commercial
projects, you must adhere to the GPL license or buy a Swiss Ephemeris
commercial license.

# Use of library

For get the version as a pointer const c_char :

```
pub extern "C" fn sweversion() -> *const c_char {
    CString::new(handler_swe02::version()).unwrap().into_raw()
}
```

For get a json with all svg and position inside as a pointer const c_char:
```
pub extern "C" fn compute(
    year: c_int,
    month: c_int,
    day: c_int,
    hourf32: c_double,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    max_size: c_double,
    path: *const c_char,
) -> *const c_char {
    let d = DataChartNatalC {
        year: year,
        month: month,
        day: day,
        hourf32: hourf32,
        hour: hour,
        min: min,
        sec: sec,
        lat: lat,
        lng: lng,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = astrology_draw_svg::chart(max_size as f32, d, &path_str);
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}
```

For all aspects (only major at this moment)

```
#[no_mangle]
pub extern "C" fn aspects() -> *const c_char {
    let data = astrology_draw_svg::all_aspects();
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}

```

For transit

```
#[no_mangle]
pub extern "C" fn compute_transit(
    year: c_int,
    month: c_int,
    day: c_int,
    hourf32: c_double,
    hour: c_int,
    min: c_int,
    sec: c_double,
    lat: c_double,
    lng: c_double,
    year_transit: c_int,
    month_transit: c_int,
    day_transit: c_int,
    hourf32_transit: c_double,
    hour_transit: c_int,
    min_transit: c_int,
    sec_transit: c_double,
    lat_transit: c_double,
    lng_transit: c_double,
    max_size: c_double,
    path: *const c_char,
) -> *const c_char {
    let d = DataChartNatalC {
        year: year,
        month: month,
        day: day,
        hourf32: hourf32,
        hour: hour,
        min: min,
        sec: sec,
        lat: lat,
        lng: lng,
    };
    let d_t = DataChartNatalC {
        year: year_transit,
        month: month_transit,
        day: day_transit,
        hourf32: hourf32_transit,
        hour: hour_transit,
        min: min_transit,
        sec: sec_transit,
        lat: lat_transit,
        lng: lng_transit,
    };
    let path_c_str = unsafe { CStr::from_ptr(path) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let data = astrology_draw_svg::chart_with_transit(
        max_size as f32,
        d,
        d_t,
        &path_str,
    );
    CString::new(serde_json::to_string(&data).unwrap())
        .unwrap()
        .into_raw()
}
```

# Version
0.1.1
* Update doc

0.1.0
* Some function for extern c use (c -> rust -> c)
