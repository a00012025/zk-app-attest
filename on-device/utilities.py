import network
import time

import config


def load_env():
    env_vars = {
        "WIFI_SSID": config.ENV_VARS['WIFI_SSID'],
        "WIFI_PASSWORD": config.ENV_VARS['WIFI_PASSWORD'],
        "API_URL": config.ENV_VARS['API_URL'],
        "USER_ID": config.ENV_VARS['USER_ID']
    }
    return env_vars

def connect_wifi(ssid, pwd):
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    wlan.connect(ssid, pwd)

    timeout = 10  # seconds
    start_time = time.time()
    while not wlan.isconnected() and time.time() - start_time < timeout:
        print("Connecting to network...")
        time.sleep(1)

    if wlan.isconnected():
        print("Connected to Wi-Fi")
        print("IP Address:", wlan.ifconfig()[0])
    else:
        print("Failed to connect to Wi-Fi")

def custom_strftime(format_string, t):
    year, month, day, hour, minute, second, _, _ = t
    return format_string.format(
        year=year, month=month, day=day,
       hour=hour, minute=minute, second=second
   )

def generate_run_id(user_id, run_time, sensor_type): # hash of some combination of user_id & when it ran & some info about data collected?
    raw_string = str(user_id) + "_" + str(run_time) + "_" + str(sensor_type)
    return raw_string
    # return hash(raw_string)

# env = load_env()


