import machine
import utime
import json
# import network

import utilities
import breathalyzer_util
import math_util
# import pico_client_tasks as cl
# import hardcoded_adjustment as ha

# FOR some context
# -> Alcohol sensor interpretation:
#     1) room: 0.6-0.8V
#     2) soft-drink breath: 1-1.3V
#     3) post alcohol breath: 1.5-2V
#     4) 30% alcohol count breath: 3V+

if __name__ == "__main__":
    # first things first
    #   -> Access the env variables
    env = utilities.load_env()
    #   -> connect to wifi
    utilities.connect_wifi(ssid=env["WIFI_SSID"], pwd=env["WIFI_PASSWORD"])

    # Run SENSORS
    #   -> Initialize the ADC (analog to digital converter) for alcohol sensor & Button
    adc = machine.ADC(26)  # GP26 corresponds to ADC0
    button_pin = machine.Pin(15, machine.Pin.IN, machine.Pin.PULL_DOWN)

    #   -> Set some run-time params
    wait_it = 0 
    while wait_it<25:
        button_state = button_pin.value()
        if button_state != 1:
            print("PRESSED")

            # FYI, particular raspberry pi used seems to thin kit's 2021, but i don;t wanna write a function to handle this...

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

            run_time = utilities.custom_strftime(
                "{year}/{month:02d}/{day:02d} {hour:02d}:{minute:02d}", 
                utime.localtime()
            )
            voltage_stats = {
                "time": run_time,
                "min": math_nerd.min,
                "max": math_nerd.max, 
                "mean": math_nerd.get_mean(),
                "std": math_nerd.get_stddev(),
                "run_id": utilities.generate_run_id(user_id=env["USER_ID"], run_time=run_time, sensor_type="alcohol_sensor")
            }
            print(voltage_stats)
            # voltage_stats = json.dumps(voltage_stats)
            # cl.send_post_request(json_data=voltage_stats, url=env.get('URL', 'default_URL'))
            del voltage_stats

        else:
            print("NOT PRESSED | {}".format(wait_it))
            utime.sleep(0.5)
            wait_it += 1
            continue
        
