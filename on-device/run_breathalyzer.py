import machine
import utime
# import network

import utilities
import breathalyzer_util
import math_util
# import hardcoded_adjustment as ha


if __name__ == "__main__":
    # first things first
    #   -> Access the env variables
    env = utilities.load_env()
    #   -> connect to wifi
    utilities.connect_wifi(ssid=env.get('SSID', 'default_SSID'), pwd=env.get('PASSWORD', 'default_PASSWORD'))

    # Run SENSORS
    #   -> Initialize the ADC (analog to digital converter) for alcohol sensor & Button
    adc = machine.ADC(26)  # GP26 corresponds to ADC0
    button_pin = machine.Pin(15, machine.Pin.IN, machine.Pin.PULL_DOWN)

    #   -> Set some run-time params
    wait_it = 0 
    while wait_it<50:
        button_state = button_pin.value()
        if button_state != 1:
            print("PRESSED")
            # now = utime.localtime()

            # q, r = divmod(now[4] + ha.__device_min_offset, 60)
            # now[4] = r
            # q, r = divmod(now[3] + ha.__device_hour_offset + q, 24)
            # now[3] = r

            # q, r = divmod(now[2] + ha.__device_day_offset + q, d_of_m)
            # now[2] = r

            # q, r = divmod(now[1] + ha.__device_month_offset + q)
            # now[1] =r

            # now[0] += ha.__device_year_offset + q


            
            math_nerd = math_util.RunningStats()

            while button_pin.value() != 1:
                # Read the analog value (0 to 65535 for 16-bit ADC)
                analog_value = adc.read_u16()
                
                # Convert the value to a voltage (assuming a 3.3V reference)
                voltage = breathalyzer_util.MQ3_analog_to_voltage(analog_value, 3.3)
                
                math_nerd.update(voltage)

                print("Analog Value:", analog_value, "| Voltage:", voltage, "V")
                utime.sleep(0.5)  # Delay for 0.5 second
            
            voltage_stats = {
                "time": utime.localtime(),
                "min": math_nerd.min,
                "max": math_nerd.max, 
                "mean": math_nerd.get_mean(),
                "std": math_nerd.get_stddev()
            }
            print(voltage_stats)
            del voltage_stats

        else:
            print("NOT PRESSED | {}".format(wait_it))
            utime.sleep(0.5)
            wait_it += 1
            continue
        
    # generate_run_id()