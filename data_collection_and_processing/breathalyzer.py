import machine
import utime

# Initialize the ADC (analog to digital converter)
adc = machine.ADC(26)  # GP26 corresponds to ADC0

while True:
    # Read the analog value (0 to 65535 for 16-bit ADC)
    analog_value = adc.read_u16()
    
    # Convert the value to a voltage (assuming a 3.3V reference)
    voltage = (analog_value / 65535.0) * 3.3
    
    print("Analog Value:", analog_value)
    print("Voltage:", voltage, "V")
    
    utime.sleep(1)  # Delay for 1 second
