use math;
use pyo3::exceptions;
use pyo3::prelude::*;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSON_Master {
    #[serde(rename(deserialize = "weather"))]
    pub weather: Vec<JSON_Weather>,

    #[serde(rename(deserialize = "main"))]
    pub temp: JSON_Temps,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSON_Weather {
    pub main: String,

    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSON_Temps {
    pub temp: f32,

    #[serde(rename = "feels_like")]
    pub feels_like: f32,

    #[serde(rename = "temp_min")]
    pub temp_min: f32,

    #[serde(rename = "temp_max")]
    pub temp_max: f32,
}

#[pyclass]
pub struct Weather {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub temp: f32,
    #[pyo3(get)]
    pub temp_max: f32,
    #[pyo3(get)]
    pub temp_min: f32,
    #[pyo3(get)]
    pub feels_like: f32,
}
#[pymethods]
impl Weather {
    #[new]
    fn new(
        name: &str,
        description: &str,
        temp: f32,
        temp_max: f32,
        temp_min: f32,
        feels_like: f32,
    ) -> Weather {
        Weather {
            name: name.to_string(),
            description: description.to_string(),
            temp,
            temp_max,
            temp_min,
            feels_like,
        }
    }
}

#[pyfunction]
fn send_request(location: String, api_key: String) -> PyResult<Weather> {
    let mut response = match reqwest::blocking::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}",
        location, api_key
    )) {
        Ok(response) => response,
        Err(_) => {
            return Err(exceptions::PyConnectionError::new_err(
                "Couldn't connect to the remote server.",
            ))
        }
    };

    let weather = match response.json::<JSON_Master>() {
        Ok(weather) => weather,
        Err(e) => {
            return Err(exceptions::PyTypeError::new_err(
                "Couldn't read the response.",
            ))
        }
    };

    let mut weather : Weather = Weather::new( &weather.weather[0].main.as_str(), weather.weather[0].description.as_str() , weather.temp.temp, weather.temp.temp_max, weather.temp.temp_min, weather.temp.feels_like);
    format_temps(&mut weather);
    Ok(weather)
}

fn format_temps(weather: &mut Weather) {
    weather.temp_max = round_temp(kelvin_to_celsius(weather.temp_max));
    weather.temp_min = round_temp(kelvin_to_celsius(weather.temp_min));
    weather.temp = round_temp(kelvin_to_celsius(weather.temp));
    weather.feels_like = round_temp(kelvin_to_celsius(weather.feels_like));
}

fn kelvin_to_celsius(temp: f32) -> f64 {
    (temp - 273.15) as f64
}

fn round_temp(temp: f64) -> f32 {
    math::round::ceil(temp, 2) as f32
}

/// A Python module implemented in Rust.
#[pymodule]
fn weather(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_request, m)?)?;
    m.add_class::<Weather>()?;
    Ok(())
}
