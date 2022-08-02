import time
import spidev
from pynput import keyboard
import matplotlib.pyplot

spi1 = spidev.SpiDev(1, 0)
spi1.max_speed_hz = 5000000 # Guessing from the datasheet max sck freq
spi1.bits_per_word = 8

global stop
stop = False

def read_adc(adc_ch, vref = 3.3):
    # Make sure ASC channel is 0 or 1:
    if adc_ch != 0:
        adc_ch = 1

    reply = spi1.xfer2([0]*4)

    # Construct single integer out of the reply (2 bytes)
    adc = 0
    for n in reply:
        adc = (adc << 8) + n

    tmp = '{:032b}'.format(adc)

    # Extract temperature information.
    # TODO: account for 14 bit signed value
    temp = 0
    i = 0
    while (i < 12):
        temp += int(tmp[i])*2**(11-i)
        i += 1

    if (int(tmp[12]) == 1):
        temp += 0.5
    if (int(tmp[13]) == 1):
        temp += 0.25

    return temp

# Report the channel 0 and channel 1 voltages to the terminal
try:
    print("Writing to temperatureData.csv")
    print("Press 'q' to quit")

    f = open("temperatureData.csv", "w")
    f.write(time.strftime("%a, %d %b %Y %H:%M:%S +0000", time.gmtime()))
    f.close()

    while not stop:
        adc_0 = read_adc(0)
        print(adc_0, "degrees C")
        strout = str(count) + ' ' + str(adc_0) + '\n'
        print("strout =", strout)
        f = open("timeVsTemp.txt", "a")
        f.write(strout)
        f.close()
        count += 1
        if not stop:
            time.sleep(0.5)

finally:
    spi1.close()
    f.close()
