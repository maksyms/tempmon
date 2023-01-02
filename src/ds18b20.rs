use crate::w1_errors::*;
use derive_more::Display;
use std::path::PathBuf;
use std::{fs, io};

static W1_PATH_PREFIX: &str = "/sys/bus/w1/devices";
static W1_PATH_SUFFIX: &str = "w1_slave";

#[derive(Debug)]
pub struct MilliCelsius(u32);

impl std::fmt::Display for MilliCelsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.3}", self.0 as f32 / 1000.0)
    }
}

impl From<MilliCelsius> for f32 {
    fn from(milli_celsius: MilliCelsius) -> Self {
        milli_celsius.0 as f32 / 1000.0
    }
}

#[derive(Debug, Display)]
pub struct DS18B20 {
    w1_id: String,
}

impl From<DS18B20> for String {
    fn from(ds18b20: DS18B20) -> Self {
        ds18b20.w1_id
    }
}

pub struct DS18B20List {
    sensors: Vec<DS18B20>,
}

impl DS18B20List {
    pub fn new() -> Result<DS18B20List, W1Error> {
        let mut sensors = Vec::new();
        for entry in fs::read_dir(W1_PATH_PREFIX)? {
            let filename = entry?.file_name().into_string().unwrap_or_default();
            if filename.contains("28-") {
                sensors.push(DS18B20 { w1_id: filename });
            }
        }

        if sensors.len() == 0 {
            return Err(W1Error::NoSensorsFound);
        }

        Ok(DS18B20List { sensors: sensors })
    }

    pub fn len(&self) -> usize {
        self.sensors.len()
    }
}

impl Iterator for DS18B20List {
    type Item = DS18B20;

    fn next(&mut self) -> Option<Self::Item> {
        self.sensors.pop()
    }
}

impl DS18B20 {
    pub fn read_raw(&self) -> io::Result<String> {
        let mut path = PathBuf::from(W1_PATH_PREFIX);
        path.push(&self.w1_id);
        path.push(W1_PATH_SUFFIX);
        fs::read_to_string(path)
    }

    pub fn read_temp(&self) -> Result<MilliCelsius, W1Error> {
        let temp_data = self.read_raw()?;
        if !temp_data.contains("YES") {
            return Err(W1Error::BadSerialConnection);
        }
        Ok(MilliCelsius(parse_temp(temp_data)?))
    }
}

fn parse_temp(temp_data: String) -> Result<u32, W1Error> {
    let t_loc = temp_data.find("t=");
    if let Some(t_loc) = t_loc {
        let (_, temp_str) = temp_data.split_at(t_loc + 2);
        return temp_str.trim().parse::<u32>().map_err(|e| e.into());
    }
    Err(W1Error::NoTemperatureFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_temp() {
        let temp_data = "6e 01 55 05 7f 7e a5 66 f2 : crc=f2 YES
6e 01 55 05 7f 7e a5 66 f2 t=22875"
            .to_string();
        assert_eq!(Ok(22875), parse_temp(temp_data));
    }
}
