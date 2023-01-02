mod config;
mod ds18b20;
mod w1_errors;
use env_logger;
use log::{debug, error, info};
//use std::collections::HashMap;
use std::{thread, time};
use ureq;

fn main() {
    env_logger::init();

    let exename = std::env::current_exe()
        .expect("Cannot get the exec path")
        .file_name()
        .expect("Cannot get the exec name")
        .to_string_lossy()
        .into_owned();

    info!(
        "Loading config from {}",
        config::TMConfig::get_config_path(exename.as_str())
    );

    let config = config::TMConfig::load(&exename);
    debug!("{:?}", config);

    if config.endpoints.is_empty() {
        error!("No endpoints configured. Exiting...");
        return;
    }

    loop {
        // Create a list of available sensors
        let sensors = ds18b20::DS18B20List::new();

        if sensors.is_err() {
            error!("No sensors found. Continuing polling...");
            thread::sleep(time::Duration::from_secs(config.report_interval));
            continue;
        }

        let sensors = sensors.expect("Unknown error, cannot unwrap sensors");
        debug!("Found {} sensors", sensors.len());

        // Report temperature for each sensor to each endpoint
        for sensor in sensors {
            let temp = sensor.read_temp();
            if temp.is_err() {
                error!("Error reading sensor: {}", temp.expect_err("Unknown error"));
                continue;
            }
            let temp = temp.expect("Unknown error, cannot unwrap temperature");
            debug!("{:?}: {} C", sensor, temp);

            for endpoint in &config.endpoints {
                report_temperature(endpoint, &sensor, &temp);
            }
        }

        thread::sleep(time::Duration::from_secs(config.report_interval));
    }
}

fn report_temperature(
    endpoint: &config::Endpoint,
    sensor: &ds18b20::DS18B20,
    temp: &ds18b20::MilliCelsius,
) {
    let mut requesturl = endpoint.url.clone();

    let sensor_idx = requesturl.find("{sensor}");
    if let Some(idx) = sensor_idx {
        requesturl.replace_range(idx..idx + 8, sensor.to_string().as_str());
    }

    let temperature_idx = requesturl.find("{temperature}");
    if let Some(idx) = temperature_idx {
        requesturl.replace_range(idx..idx + 13, temp.to_string().as_str());
    }

    let mut request = ureq::request(&endpoint.method, &requesturl);

    if let Some(headers) = &endpoint.headers {
        for (key, value) in headers {
            request = request.set(key, value);
        }
    }

    let response = if let Some(body) = &endpoint.body {
        let mut body = body.clone();

        let sensor_idx = body.find("{sensor}");
        if let Some(idx) = sensor_idx {
            body.replace_range(idx..idx + 8, sensor.to_string().as_str());
        }

        let temperature_idx = body.find("{temperature}");
        if let Some(idx) = temperature_idx {
            body.replace_range(idx..idx + 13, temp.to_string().as_str());
        }

        request.send_string(&body)
    } else {
        request.call()
    };

    if response.is_err() {
        error!("Error: {}", response.expect_err("Unknown error"));
    } else {
        debug!(
            "Response: {}",
            response
                .expect("Unknown error")
                .into_string()
                .expect("Unknown error")
        );
    }
}
