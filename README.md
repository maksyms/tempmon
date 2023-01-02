# TempMon

This is just a toy program to regularly upload the values from DS18B20 sensor(s) connected to Raspberry Pi 1 Model B to an endpoint of your choice through GET or POST requests. For other Raspberry Pis, change `Cross.toml` to the required platform.

- Inspired by [awendland/rpi-ds18b20-rust](https://github.com/awendland/rpi-ds18b20-rust).
- Runs on the amazing [DietPi](https://dietpi.com/). Some extra setup to enable SPI on GPIO pin 4 was needed in `/boot/config.txt` - i.e., `dtparam=spi=on` and `dtoverlay=w1-gpio,gpiopin=4`, in addition to adding `w1-gpio` and `w1-therm` to `/etc/modules`.
- DS18B20 were connected in parallel with 4.7kOhm single resistor between data and VCC.
- Requires [cross](https://github.com/cross-rs/cross) and `docker` to compile. More documentation is coming.
- The configuration is in `/home/dietpi/.config/tempmon/tempmon.toml`.
- Can run as a daemon using `systemctl` - just copy `tempmon.service` to `/etc/systemd/system/tempmon.service` and run things like `sudo systemctl daemon-reload; sudo systemctl enable tempmon`
