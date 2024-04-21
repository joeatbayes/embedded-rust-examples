import math

def calculate_capacitance(discharge_time_us, charge_voltage, lower_voltage, resistor):
  """
  This function calculates capacitance in picoFarads given discharge time in microseconds,
  charging voltage, lower voltage, and resistor value.

  Args:
      discharge_time_us (float): Discharge time in microseconds.
      charge_voltage (float): Charging voltage in Volts.
      lower_voltage (float): Lower voltage in Volts.
      resistor (float): Resistor value in Ohms.

  Returns:
      float: Capacitance in picoFarads.
  """

  # Convert time to seconds
  discharge_time = discharge_time_us / 1e6

  # Calculate time constant (tau)
  tau = -discharge_time / math.log(lower_voltage / charge_voltage, math.exp(1))

  # Calculate capacitance
  capacitance = tau / resistor

  # Convert capacitance to picoFarads
  capacitance_pf = capacitance * 1e12

  return capacitance_pf

# Example usage
discharge_time_us = 29.105745  # microseconds
charge_voltage = 3  # Volts
lower_voltage = 0.7  # Volts
resistor = 5e6  # Ohms

capacitance_pf = calculate_capacitance(discharge_time_us, charge_voltage, lower_voltage, resistor)

print(f"Capacitance: {capacitance_pf:.2f} picoFarads")