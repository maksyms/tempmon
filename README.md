# TempMon

This is just a toy program to regularly upload the values from DS18B20 sensor(s) connected to Raspberry Pi to an endpoint of your choice through GET or POST requests.

- Inspired by [awendland/rpi-ds18b20-rust](https://github.com/awendland/rpi-ds18b20-rust)
- Runs on the amazing [DietPi](https://dietpi.com/)
- Requires `cross` and `docker` to compile. More documentation is coming.
- The configuration is in `/home/dietpi/.config/tempmon/tempmon.toml`.
- Can run as a daemon using `systemctl` - just copy `tempmon.service` to `/etc/systemd/system/tempmon.service` and run things like `sudo systemctl daemon-reload; sudo systemctl enable tempmon`
