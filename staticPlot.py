####################################################################################
# Firmware for HotPi board
# Allows a Raspberry Pi 4b to control 6 MAX31855 thermocouple-to-digital converters
# Records temperature data from the spi device to a file, then graphs it
# First, enable spi1 on the Raspberry Pi
####################################################################################

# TODO: Support all 6 thermocouples being used in parallel instead of just 1

import time
import spidev
from pynput import keyboard
import matplotlib.pyplot as plt

spi1 = spidev.SpiDev(1, 0)
spi1.max_speed_hz = 5000000 # Guessing from the datasheet max sck freq
spi1.bits_per_word = 8

global stop
stop = False

fig = plt.figure()
ax = fig.add_subplot(1,1,1)

def on_press(key):
    global stop
    try:
        if key.char == 'q':
            stop = True
            print()
    except AttributeError:
        print('\nspecial key {0} pressed\n'.format(key))

def make_plot():
    xar = []
    yar = []

    dataFile = open("temperatureData.csv", "r")
    data = dataFile.read()
    dataFile.close()
    data_array = data.split('\n')

    for each_line in data_array:
        if len(each_line) > 1 and len(each_line) < 20:
            x,y = each_line.split(',')
            xar.append(int(x))
            yar.append(float(y))

    ax.clear()
    ax.plot(xar, yar)
    plt.show()

def read_adc():
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

# Create the keyboard listener thread
listener = keyboard.Listener( on_press=on_press )
listener.start()

# Report the channel 0 and channel 1 voltages to the terminal
try:
    print("Writing to temperatureData.csv")
    print("Press 'q' to quit")

    f = open("temperatureData.csv", "w")
    f.write(time.strftime("%a, %d %b %Y %H:%M:%S +0000", time.gmtime()))
    f.close()

    count = 0

    while not stop:
        adc_0 = read_adc()
        print(adc_0, "degrees C")

        strout = str(count) + ',' + str(adc_0) + '\n'
        f = open("temperatureData.csv", "a")
        f.write(strout)
        f.close()

        count += 1

        if not stop:
            time.sleep(1)

    make_plot()

finally:
    spi1.close()
    f.close()
