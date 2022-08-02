import time
import spidev

spi1 = spidev.SpiDev(1, 0)
# If we want more spi devices, it would look like the following:
# spi[n] = spidev.SpiDev([bus], [cs])
spi1.max_speed_hz = 5000000 # Guessing from the datasheet max sck freq
spi1.bits_per_word = 8

def read_adc(adc_ch, vref = 3.3):
    # Make sure ASC channel is 0 or 1:
    if adc_ch != 0:
        adc_ch = 1

    reply = spi1.xfer2([0]*4)
    print("reply:", reply) # For testing

    # Construct single integer out of the reply (2 bytes)
    adc = 0
    for n in reply:
        adc = (adc << 8) + n

    print("adc:", adc) # For testing

    tmp = '{:032b}'.format(adc)

    print("tmp =", tmp)
    
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
    while True:
        adc_0 = read_adc(0)
        print("CH 0:", round(adc_0, 2), "degrees C")
        time.sleep(0.2)

finally:
    spi1.close()
    # GPIO.cleanup()
