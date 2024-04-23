/* Example of changing pin direction on ESP32
   Measuring capacitance by first Charing variable capacitor by driving pin
   high for a charging interval then converting pin into high inpedance input
   measuring time for it to drop below logic low. 

   Tested on RainAmp Soil Sensor Baord 2020-04-20
   ESPS3 Mini 1 - 4MB Flash, 2MB PSRAM

   Arduino Board Selected: ESP32S3 Dev Module
     USB CDC Enabled On Boot - Enabled
     CPU Frequency 80MHz
     CPU Debug Level WARN
     CPU Firmware 4MB
     CPU PSRAM - QSPI RAM 
  
  Wiring
    GPIO1 -> SENS_OSC
    GPIO1 -> 8M Resistor -> GND  
    GPIO11 -> 4K Reistor -> GND
    SENS_GRND -> GND
*/

#define LED_PIN 11
#define SENSE_PIN 1
#define NUM_PASS 100
#define FULL_DISCHARGE_MS 2
#define CHARGE_US 2000
#define MAX_COUNT_DISCHARG 1000000
float avgUs = CHARGE_US;
// the setup routine runs once when you press reset:
void setup() {
  // initialize serial communication at 9600 bits per second:
  Serial.begin(115200);
  pinMode(LED_PIN, OUTPUT);
  pinMode(SENSE_PIN, OUTPUT);
  digitalWrite(LED_PIN, HIGH);
  digitalWrite(SENSE_PIN, LOW);
}

// the loop routine runs over and over again forever:
void loop() {
  // Give Sense Pin Time to fully discharge
  pinMode(SENSE_PIN, OUTPUT);
  digitalWrite(SENSE_PIN, LOW);
  delay(FULL_DISCHARGE_MS);
  digitalWrite(LED_PIN, HIGH);
  float chargeUs = (long) (avgUs * 2);
  // Oversample collection of time to discharge
  long start = micros();
  unsigned long sum = 0;
  for (int pass=0; pass < NUM_PASS; pass++)
  {
    pinMode(SENSE_PIN, OUTPUT);
    digitalWrite(SENSE_PIN, HIGH);     
    delayMicroseconds(CHARGE_US);
    pinMode(SENSE_PIN, INPUT);
    // Wait for capacitor to drain through
    // Bleed resistor
    unsigned long cnt = 0;
    while((digitalRead(SENSE_PIN) == HIGH) && (cnt < MAX_COUNT_DISCHARG)){
      cnt += 1;
    }
    sum  += cnt;
  }
  unsigned long avg = sum / NUM_PASS;  
  unsigned long elap = micros() - start;
  unsigned long elapDrop = elap - (NUM_PASS * CHARGE_US);
  float avgUs = (float)elapDrop / (float) NUM_PASS;
  digitalWrite(LED_PIN, LOW);

  Serial.printf("avg Cnt %ld  Elap Avg %8.5f us\n", avg, avgUs);
  
  delay(1500);  // delay in between reads for stability
}
