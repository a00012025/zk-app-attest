import machine
import utime
# import network

import utilities
import breathalyzer


if __name__ == "__main__":
    # first things first
    #   -> Access the env variables
    env = utilities.load_env()
    #   -> connect to wifi
    utilities.connect_wifi(ssid=env.get('SSID', 'default_SSID'), pwd=env.get('PASSWORD', 'default_PASSWORD'))

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
        voltage = breathalyzer.MQ3_analog_to_voltage(analog_value, 3.3)
        
        print("i={} | Analog Value:".format(i), analog_value, "| Voltage:", voltage, "V")

        utime.sleep(increment_time)  # Delay for 1 second
        i = i+increment_time

    # generate_run_id()

