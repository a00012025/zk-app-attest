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
