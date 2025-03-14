// raspi_gpio.cpp
//  splashkit
// Created by Aditya Parmar on 20/01/2024.
// Copyright © 2024 XQuestCode. All rights reserved.
#include "raspi_gpio.h"
#include "gpio_driver.h"
#include "easylogging++.h"
#include "types.h"
#include <iostream>
using namespace std;

namespace splashkit_lib
{
    // Each index points to PIN_1, PIN_2, PIN_3, etc.
    int BCMpinData[] = {
        -1, -1, 2, -1, 3, -2, 4, 14, -2, 15, 17, 18, 27, -2, 22, 23, -1, 24, 10, -2, 9, 25, 11, 8, -2, 7, 0, 1, 5, -2, -6, 12, 13, -2, 19, 16, 26, 20, -2, 21};

    int boardToBCM(gpio_pin pin)
    {
        int bcmPinResult = PI_BAD_GPIO;
        if (pin >= PIN_1 && pin <= PIN_40)
        {
            bcmPinResult = BCMpinData[static_cast<int>(pin) - static_cast<int>(PIN_1)];
        }

        // Pins 0 and 1 are EEPROM pins and should not be written to
        // See Page 9. of Raspberry Pi 4B Datasheet:
        // https://datasheets.raspberrypi.com/rpi4/raspberry-pi-4-datasheet.pdf
        // Archive Link:
        // https://web.archive.org/web/20240901170108/https://datasheets.raspberrypi.com/rpi4/raspberry-pi-4-datasheet.pdf
        if(bcmPinResult < 2)
        {
            std::string extra_text = " Pin is a";
    	    extra_text += (bcmPinResult >= 0) ? "n EEPROM Pin, using this could corrupt the bootloader." :
	    		          (bcmPinResult == -1) ? " POWER line." :
                          (bcmPinResult == -2) ? " GROUND line." : "n Unknown Pin Type.";
            LOG(ERROR) << sk_gpio_error_message(PI_BAD_GPIO) + extra_text;
            return PI_BAD_GPIO;
        }
        else
        {
            return bcmPinResult;
        }
    }

    bool has_gpio()
    {
#ifdef RASPBERRY_PI
        return true;
#else
        return false;
#endif
    }

    // Initialize GPIO resources
    void raspi_init()
    {
#ifdef RASPBERRY_PI
        sk_gpio_init();
#else
        cout << "GPIO not supported on this platform" << endl;
#endif
    }

    // Set the mode of the given pin
    void raspi_set_mode(gpio_pin pin, gpio_pin_mode mode)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)
        {
            sk_gpio_set_mode(bcmPin, static_cast<int>(mode));
        }
#else
        cout << "Unable to set mode - GPIO not supported on this platform" << endl;
#endif
    }

    gpio_pin_mode raspi_get_mode(gpio_pin pin)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)    
        {
            return static_cast<pin_modes>(sk_gpio_get_mode(bcmPin));
        }
        return GPIO_DEFAULT_MODE;
#else
        return GPIO_DEFAULT_MODE;
        cout << "Unable to get mode - GPIO not supported on this platform" << endl;
#endif
    }

    // Write a value to the given pin
    void raspi_write(gpio_pin pin, gpio_pin_value value)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) 
        {
            sk_gpio_write(bcmPin, static_cast<int>(value));
        }
#else
        cout << "Unable to write pin - GPIO not supported on this platform" << endl;
#endif
    }

    // Read the value of the given pin
    gpio_pin_value raspi_read(gpio_pin pin)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)
        {
            return static_cast<pin_values>(sk_gpio_read(bcmPin));
        }
        return GPIO_DEFAULT_VALUE;
#else
        cout << "Unable to read pin - GPIO not supported on this platform" << endl;
        return GPIO_DEFAULT_VALUE;
#endif
    }

    void raspi_set_pull_up_down(gpio_pin pin, pull_up_down pud)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)
        {
            sk_gpio_set_pull_up_down(bcmPin, static_cast<int>(pud));
        }
#else
        cout << "Unable to set pull up/down - GPIO not supported on this platform" << endl;
#endif
    }

    void raspi_set_pwm_range(gpio_pin pin, int range)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)
        {
            sk_set_pwm_range(bcmPin, range);
        }
#else
        cout << "Unable to set pwm range - GPIO not supported on this platform" << endl;
#endif
    }

    void raspi_set_pwm_frequency(gpio_pin pin, int frequency)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)
        {
            sk_set_pwm_frequency(bcmPin, frequency);
        }
#else
        cout << "Unable to set pwm frequency - GPIO not supported on this platform" << endl;
#endif
    }

    void raspi_set_pwm_dutycycle(gpio_pin pin, int dutycycle)
    {
#ifdef RASPBERRY_PI
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2)
        {
            sk_set_pwm_dutycycle(bcmPin, dutycycle);
        }
#else
        cout << "Unable to set pwm dutycycle - GPIO not supported on this platform" << endl;
#endif
    }
	
	int raspi_spi_open(int channel, int speed, int spi_flags)
    {
#ifdef RASPBERRY_PI
        int handle = -1;
	    handle = sk_spi_open(channel, speed, spi_flags);
        return handle;
#else
        cout << "Unable to open SPI interface - GPIO not supported on this platform" << endl;
        return -1;
#endif
    }

    int raspi_spi_close(int handle)
    {
#ifdef RASPBERRY_PI
        return sk_spi_close(handle);
#else
        cout << "Unable to close SPI interface - GPIO not supported on this platform" << endl;
        return -1;
#endif
    }

    int raspi_spi_transfer(int handle, string sendBuf, string recvBuf, int count)
    {
#ifdef RASPBERRY_PI
        return sk_spi_transfer(handle, sendBuf, recvBuf, count);
#else
        cout << "Unable to transfer through SPI - GPIO not supported on this platform" << endl;
        return -1;
#endif
    }

    // Cleanup GPIO resources
    void raspi_cleanup()
    {
#ifdef RASPBERRY_PI
        LOG(INFO) << "Cleaning GPIO pins";
       	sk_gpio_clear_bank_1();
    	sk_gpio_cleanup();
#else
        cout << "Unable to set cleanup - GPIO not supported on this platform" << endl;
#endif
    }

    connection remote_raspi_init(const string &name, const string &host, unsigned short int port)
    {
        return sk_remote_gpio_init(name, host, port);
    }

    void remote_raspi_set_mode(connection pi, gpio_pin pin, gpio_pin_mode mode)
    {
        int bcmPin = boardToBCM(pin);
        if (bcmPin >= 2) sk_remote_gpio_set_mode(pi, bcmPin, mode);
    }

    gpio_pin_mode remote_raspi_get_mode(connection pi, gpio_pin pin)
    {
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) return static_cast<gpio_pin_mode>(sk_remote_gpio_get_mode(pi, bcmPin));
        else return GPIO_DEFAULT_MODE;
    }

    void remote_raspi_set_pull_up_down(connection pi, gpio_pin pin, pull_up_down pud)
    {
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) sk_remote_gpio_set_pull_up_down(pi, bcmPin, pud);
    }

    void remote_raspi_write(connection pi, gpio_pin pin, gpio_pin_value value)
    {
        int bcmPin = boardToBCM(pin);
        
        if(bcmPin >= 2) sk_remote_gpio_write(pi, bcmPin, value);
    }

    gpio_pin_value remote_raspi_read(connection pi, gpio_pin pin)
    {
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) return static_cast<gpio_pin_value>(sk_remote_gpio_read(pi, bcmPin));
        else return GPIO_DEFAULT_VALUE;
    }

    void remote_raspi_set_pwm_range(connection pi, gpio_pin pin, int range)
    {
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) sk_remote_set_pwm_range(pi, bcmPin, range);
    }

    void remote_raspi_set_pwm_frequency(connection pi, gpio_pin pin, int frequency)
    {
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) sk_remote_set_pwm_frequency(pi, bcmPin, frequency);
    }

    void remote_raspi_set_pwm_dutycycle(connection pi, gpio_pin pin, int dutycycle)
    {
        int bcmPin = boardToBCM(pin);
        if(bcmPin >= 2) sk_remote_set_pwm_dutycycle(pi, bcmPin, dutycycle);
    }

    bool remote_raspi_cleanup(connection pi)
    {
        return sk_remote_gpio_cleanup(pi);
    }
}
