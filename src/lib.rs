use pyo3::prelude::*;
use reqwest::blocking;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    #[pyo3(get)]
    #[serde(rename(deserialize = "weather"))]
    pub weather: Vec<Weather>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "main"))]
    pub temp: Temp,
}
#[pyclass]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    #[pyo3(get)]
    pub main: String,
    #[pyo3(get)]
    pub description: String,
}

#[pyclass]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temp {
    #[pyo3(get)]
    pub temp: f32,
    #[pyo3(get)]
    #[serde(rename = "feels_like")]
    pub feels_like: f32,
    #[pyo3(get)]
    #[serde(rename = "temp_min")]
    pub temp_min: f32,
    #[pyo3(get)]
    #[serde(rename = "temp_max")]
    pub temp_max: f32,
}

#[pyfunction]
fn send_request(location: String, api_key: String) -> PyResult<Master> {
    let response = reqwest::blocking::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}",
        location, api_key
    ))
    .unwrap();
    let mut weather = response.json::<Master>().unwrap();
    println!("{:#?}", weather);
    convert_temps(&mut weather);
    println!("{:#?}", weather);
    Ok(weather)
}
fn convert_temps(weather: &mut Master) {
    weather.temp.temp_max = kelvin_to_celcius(weather.temp.temp_max);
    weather.temp.temp_min = kelvin_to_celcius(weather.temp.temp_min);
    weather.temp.temp = kelvin_to_celcius(weather.temp.temp);
    weather.temp.feels_like = kelvin_to_celcius(weather.temp.feels_like)
}

fn kelvin_to_celcius(temp: f32) -> f32 {
    temp - 273.15
}

/// A Python module implemented in Rust.
#[pymodule]
fn weather_cli(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_request, m)?)?;
    m.add_class::<Master>()?;
    m.add_class::<Weather>()?;
    m.add_class::<Temp>()?;
    Ok(())
}
