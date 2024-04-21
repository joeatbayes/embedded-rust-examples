import math

# Capacitor value in Farads
C = 4e-12  # 4 pico farads

# Resistor value in Ohms
R = 5e6  # 5 mohm

# Voltage difference in Volts
delta_V = 3 - 0.7

# Discharge time constant (tau) in seconds
tau = R * C

# Time to discharge from 3V to 0.7V (6 decimal places)
discharge_time = -tau * math.log(0.7 / 3, math.exp(1))

# Convert time to microseconds
discharge_time_us = discharge_time * 1e6

print(f"Discharge time: {discharge_time_us:.6f} microseconds")