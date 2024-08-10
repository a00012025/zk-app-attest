import machine
import utime
# import network

from utilities import load_env, connect_wifi, generate_run_id

# def load_env(filename='.env'):
#     env_vars = {}
#     try:
#         with open(filename, 'r') as file:
#             for line in file:
#                 if '=' in line:
#                     key, value = line.strip().split('=', 1)
#                     env_vars[key] = value
#     except OSError as e:
#         print("Error reading .env file:", e)
#     return env_vars

# def connect_wifi(ssid, pwd):
#     wlan = network.WLAN(network.STA_IF)
#     wlan.active(True)
#     wlan.connect(ssid, pwd)

#     timeout = 10  # seconds
#     start_time = utime.time()
#     while not wlan.isconnected() and utime.time() - start_time < timeout:
#         print("Connecting to network...")
#         utime.sleep(1)

#     if wlan.isconnected():
#         print("Connected to Wi-Fi")
#         print("IP Address:", wlan.ifconfig()[0])
#     else:
#         print("Failed to connect to Wi-Fi")

# 1) Collect raw data
# 2) Do simple processing to convert to a readable output -> like am I drunk or not (Not Drunk, Buzzed, Drunk, Wasted, About to Pass out)

def MQ3_analog_to_voltage(analog_value, voltage_info): # Turn raw analog sensor data -> voltage
    """
    analog_value: raw analog output value - float
    voltage_info: usually 3.3V or 5.5V - float
    """
    return (analog_value / 65535.0) * voltage_info

# TODO: tune to hardcode the eq
def voltage_to_alcohol_concentration(voltage): # voltage -> alcohol concentration
    # Observation -> 0.6~ 0.8V is room
    # soft drinks get 1~1.3V
    # hand sanitizer?
    alcohol_concentration = voltage
    return alcohol_concentration

# def alcoho


if __name__ == "__main__":
    # first things first
    #   -> Access the env variables
    env = load_env()
    #   -> connect to wifi
    connect_wifi(ssid=env.get('SSID', 'default_SSID'), pwd=env.get('PASSWORD', 'default_PASSWORD'))

    # Run DA SENSORS
    #   -> Initialize the ADC (analog to digital converter)
    adc = machine.ADC(26)  # GP26 corresponds to ADC0

    #   -> Set some run-time params
    max_sec = 60*0.1
    i = 0
    increment_time = 1
    while i < max_sec:
        # Read the analog value (0 to 65535 for 16-bit ADC)
        analog_value = adc.read_u16()
        
        # Convert the value to a voltage (assuming a 3.3V reference)
        voltage = MQ3_analog_to_voltage(analog_value, 3.3)
        
        print("i={} | Analog Value:".format(i), analog_value, "| Voltage:", voltage, "V")

        utime.sleep(increment_time)  # Delay for 1 second
        i = i+increment_time

    # generate_run_id()

