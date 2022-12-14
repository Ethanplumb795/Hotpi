####################################################################################
# Firmware for HotPi board
# Allows a Raspberry Pi 4b to control 6 MAX31855 thermocouple-to-digital converters
# Records temperature data and graphs it simultaneously
# Enable spi1 on the Raspberry Pi before use
####################################################################################

# TODO: Support all 6 thermocouples being used in parallel instead of just 1

# Imports
import time
import spidev
from pynput import keyboard
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import psutil


# Init variables

stop = False
count = 0

# Init plot
fig = plt.figure()
ax = plt.subplot()


# Function Definitions

def on_press(key):
    global stop
    try:
        if key.char == 'q':
            stop = True
            print()
    except AttributeError:
        print('\nspecial key {0} pressed\n'.format(key))

def update_plot(frame):
    xar = []
    yar = []

    read_temp()

    dataFile = open("temperatureData.csv", "r")
    data = dataFile.read()
    dataFile.close()
    data_array = data.split('\n')

    for each_line in data_array:
        if len(each_line) > 1 and len(each_line) < 20:
            x,y = each_line.split(',')
            xar.append(int(x))
            yar.append(float(y))

    ax.cla()
    ax.plot(xar, yar)

def read_adc():
    global count
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

def read_temp():
    global count

    adc_0 = read_adc()
    print(adc_0, "degrees C")

    strout = str(count) + ',' + str(adc_0) + '\n'
    f = open("temperatureData.csv", "a")
    f.write(strout)
    f.close()

    count += 1


# Begin "main":

# Init spi
spi1 = spidev.SpiDev(1, 0)
spi1.max_speed_hz = 5000000 # Guessing from the datasheet max sck freq
spi1.bits_per_word = 8

# Create the keyboard listener thread
listener = keyboard.Listener( on_press=on_press )
listener.start()

try:
    print("Writing to temperatureData.csv")

    f = open("temperatureData.csv", "w")
    f.write(time.strftime("%a, %d %b %Y %H:%M:%S +0000\n", time.gmtime()))
    f.close()

    ani = FuncAnimation(fig, update_plot, interval=1000)

    plt.show()

finally:
    spi1.close()
    f.close()
