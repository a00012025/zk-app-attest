import network
import time

def load_env(filename='.env'):
    env_vars = {}
    try:
        with open(filename, 'r') as file:
            for line in file:
                if '=' in line:
                    key, value = line.strip().split('=', 1)
                    env_vars[key] = value
    except OSError as e:
        print("Error reading .env file:", e)
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


def generate_run_id(user_id, run_time, sensor_type): # hash of some combination of user_id & when it ran & some info about data collected?
    raw_string = str(user_id) + str(run_time) + str(sensor_type)
    return raw_string
    # return hash(raw_string)

# env = load_env()


