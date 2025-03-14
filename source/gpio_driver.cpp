// gpio_driver.cpp
// This file is part of the SplashKit Core Library.
// Copyright (©) 2024 Aditya Parmar. All Rights Reserved.

#include "network_driver.h"
#include "gpio_driver.h"
#include "easylogging++.h"

#include <string>
#include <iostream>
#include <cstdlib> // Add this line to include the necessary header for the exit() function

#include <cstring>
#ifdef RASPBERRY_PI
#include "pigpiod_if2.h"
#endif


using namespace std;
// Use https://abyz.me.uk/rpi/pigpio/pdif2.html for local command reference
//   Archive Link: https://web.archive.org/web/20240423160241/https://abyz.me.uk/rpi/pigpio/pdif2.html
//
// Use https://abyz.me.uk/rpi/pigpio/sif.html for remote command reference
//   Archive Link: https://web.archive.org/web/20240423160319/https://abyz.me.uk/rpi/pigpio/sif.html
namespace splashkit_lib
{
    #ifdef RASPBERRY_PI
    int pi = -1;

    // Check if pigpio_init() has been called before any other GPIO functions
    bool check_pi()
    {
        if (pi < 0)
        {
            LOG(ERROR) << sk_gpio_error_message(pi);
            return false;
        }
        else return true;
    }

    // Initialize the GPIO library
    int sk_gpio_init()
    {
        pi = pigpio_start(0, 0);
        return pi;
    }

    // Read the value of a GPIO pin
    int sk_gpio_read(int pin)
    {
        if (check_pi())
        {
            int result = gpio_read(pi, pin);
            if (result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
            return result;
        }
        else
        {
            return GPIO_DEFAULT_VALUE;
        }
    }

    // Write a value to a GPIO pin
    void sk_gpio_write(int pin, int value)
    {
        if (check_pi())
        {
            int result = gpio_write(pi, pin, value);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
        }
    }

    // Set the mode of a GPIO pin
    void sk_gpio_set_mode(int pin, int mode)
    {
        if(check_pi())
        {
            int result = set_mode(pi, pin, mode);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
        }
    }

    int sk_gpio_get_mode(int pin)
    {
        if(check_pi())
        {
            int result = get_mode(pi, pin);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
            return result;
        }
    }
    void sk_gpio_set_pull_up_down(int pin, int pud)
    {
        if(check_pi())
        {
            int result = set_pull_up_down(pi, pin, pud);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
        }
    }

    void sk_set_pwm_range(int pin, int range)
    {
        if(check_pi())
        {
            int result = set_PWM_range(pi, pin, range);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
        }
    }
    void sk_set_pwm_frequency(int pin, int frequency)
    {
        if(check_pi())
        {
            int result = set_PWM_frequency(pi, pin, frequency);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
        }
    }

    void sk_set_pwm_dutycycle(int pin, int dutycycle)
    {
        if(check_pi())
        {
            int result = set_PWM_dutycycle(pi, pin, dutycycle);
            if(result < 0)
            {
                LOG(ERROR) << sk_gpio_error_message(result);
            }
        }
    }

    void sk_gpio_clear_bank_1()
    {
        if(check_pi())
        {
            clear_bank_1(pi, PI4B_GPIO_BITMASK);
        }
    }

    // Cleanup the GPIO library
    void sk_gpio_cleanup()
    {
        if(check_pi())
        {
            pigpio_stop(pi);
        }
    }
    
    int sk_spi_open(int channel, int speed, int spi_flags)
    {
        if(check_pi())
            return spi_open(pi, channel, speed, spi_flags);
        else
            return -1;
    }

    int sk_spi_close(int handle)
    {
        if(check_pi())
            return spi_close(pi, handle);
        else
            return -1;
    }

    int sk_spi_transfer(int handle, char *sendBuf, char *recvBuf, int count)
    {
        if(check_pi())
            return spi_xfer(pi, handle, sendBuf, recvBuf, count);
        else
            return -1;
    }	

    #endif

    connection sk_remote_gpio_init(std::string name, const std::string &host, unsigned short int port)
    {
        return open_connection(name, host, port);
    }

    void sk_remote_gpio_set_mode(connection pi, int pin, int mode)
    {
        sk_pigpio_cmd_t set_cmd;
        set_cmd.cmd_code = GPIO_CMD_SET_MODE;
        set_cmd.param1 = pin;
        set_cmd.param2 = mode;

        sk_gpio_send_cmd(pi, set_cmd);
    }

    int sk_remote_gpio_get_mode(connection pi, int pin)
    {
        sk_pigpio_cmd_t get_cmd;
        get_cmd.cmd_code = GPIO_CMD_GET_MODE;
        get_cmd.param1 = pin;

        return sk_gpio_send_cmd(pi, get_cmd);
    }

    void sk_remote_gpio_set_pull_up_down(connection pi, int pin, int pud)
    {
        sk_pigpio_cmd_t set_pud_cmd;
        set_pud_cmd.cmd_code = GPIO_CMD_SET_PUD;
        set_pud_cmd.param1 = pin;
        set_pud_cmd.param2 = pud;

        sk_gpio_send_cmd(pi, set_pud_cmd);
    }

    int sk_remote_gpio_read(connection pi, int pin)
    {
        sk_pigpio_cmd_t read_cmd;
        read_cmd.cmd_code = GPIO_CMD_READ;
        read_cmd.param1 = pin;

        return sk_gpio_send_cmd(pi, read_cmd);
    }

    void sk_remote_gpio_write(connection pi, int pin, int value)
    {
        sk_pigpio_cmd_t write_cmd;
        write_cmd.cmd_code = GPIO_CMD_WRITE;
        write_cmd.param1 = pin;
        write_cmd.param2 = value;

        sk_gpio_send_cmd(pi, write_cmd);
    }

    void sk_remote_set_pwm_range(connection pi, int pin, int range)
    {
        sk_pigpio_cmd_t set_range_cmd;
        set_range_cmd.cmd_code = GPIO_CMD_SET_PWM_RANGE;
        set_range_cmd.param1 = pin;
        set_range_cmd.param2 = range;

        sk_gpio_send_cmd(pi, set_range_cmd);
    }

    void sk_remote_set_pwm_frequency(connection pi, int pin, int frequency)
    {
        sk_pigpio_cmd_t set_freq_cmd;
        set_freq_cmd.cmd_code = GPIO_CMD_SET_PWM_FREQ;
        set_freq_cmd.param1 = pin;
        set_freq_cmd.param2 = frequency;

        sk_gpio_send_cmd(pi, set_freq_cmd);
    }

    void sk_remote_set_pwm_dutycycle(connection pi, int pin, int dutycycle)
    {
        sk_pigpio_cmd_t set_dutycycle_cmd;
        set_dutycycle_cmd.cmd_code = GPIO_CMD_SET_PWM_DUTYCYCLE;
        set_dutycycle_cmd.param1 = pin;
        set_dutycycle_cmd.param2 = dutycycle;

        sk_gpio_send_cmd(pi, set_dutycycle_cmd);
    }

    void sk_remote_clear_bank_1(connection pi)
    {
        sk_pigpio_cmd_t clear_bank_cmd;
        clear_bank_cmd.cmd_code = GPIO_CMD_CLEAR_BANK_1;
        clear_bank_cmd.param1 = PI4B_GPIO_BITMASK;

        sk_gpio_send_cmd(pi, clear_bank_cmd);
    }

    bool sk_remote_gpio_cleanup(connection pi)
    {
        if(!is_connection_open(pi))
        {
            LOG(ERROR) << "Remote GPIO: Connection not open.";
            return false;
        }
        LOG(INFO) << "Cleaning Pins on Remote Pi Named: " << pi->name << endl;
        sk_remote_clear_bank_1(pi);
        return close_connection(pi);
    }

    int sk_gpio_send_cmd(connection pi, sk_pigpio_cmd_t &cmd)
    {
        if(!is_connection_open(pi))
        {
            LOG(ERROR) << sk_gpio_error_message(PIGIF_ERR_BAD_CONNECT); 
            return PIGIF_ERR_BAD_CONNECT;
        }

        if(pi->protocol == TCP)
        {
            int num_send_bytes = sizeof(cmd);

            std::vector<char> buffer(num_send_bytes);
            memcpy(buffer.data(), &cmd, num_send_bytes);

            if(sk_send_bytes(&pi->socket, buffer.data(), num_send_bytes)) 
            {
                int num_bytes_recv = sk_read_bytes(&pi->socket, buffer.data(), num_send_bytes); 
                if(num_bytes_recv == num_send_bytes) 
                {
                    sk_pigpio_cmd_t resp;
                    memcpy(&resp, buffer.data(), num_send_bytes);
                    
                    // We cast it back to a signed type so we can get the negative error codes.
                    int32_t result = static_cast<int32_t>(resp.result);

                    if (result < 0)
                    {
                        LOG(ERROR) << sk_gpio_error_message(result);
                    }

                    return result;
                }
                else
                {
                    LOG(ERROR) << sk_gpio_error_message(PIGIF_ERR_BAD_RECV);
                    return PIGIF_ERR_BAD_RECV;
                }
            }
            else
            {
                LOG(ERROR) << sk_gpio_error_message(PIGIF_ERR_BAD_SEND);
                return PIGIF_ERR_BAD_SEND;
            }
        }
        else
        {
            LOG(ERROR) << "Remote GPIO: Connection has UDP Protocol";
            return -1;
        }
    }
    
    std::string sk_gpio_error_message(int error_code)
    {
        switch (error_code)
        {
            case PI_INIT_FAILED:
                return "GPIO initialization failed. Please check your setup and try again.";
            case PI_BAD_USER_GPIO:
            case PI_BAD_GPIO:
                return "Invalid GPIO pin number.";
            case PI_BAD_MODE:
                return "Invalid GPIO mode. Valid modes are 0-7.";
            case PI_BAD_LEVEL:
                return "Invalid GPIO level. Valid levels are 0 (LOW) or 1 (HIGH).";
            case PI_BAD_PUD:
                return "Invalid pull-up/down configuration. Valid options are 0 (OFF), 1 (Pull-down), 2 (Pull-up).";
            case PI_BAD_DUTYCYCLE:
                return "Invalid PWM duty cycle. Duty cycle must be between 0 and the range value (default 255).";
            case PI_BAD_DUTYRANGE:
                return "Invalid PWM range. Range must be between 25 and 40000.";
            case PIGIF_ERR_BAD_SEND:
                return "Failed to send command to remote GPIO daemon (pigpiod).";
            case PIGIF_ERR_BAD_RECV:
                return "Failed to receive response from remote GPIO daemon (pigpiod).";
            case PIGIF_ERR_BAD_CONNECT:
                return "Failed to connect to remote GPIO daemon (pigpiod).";
            default:
                return "Unknown error code " + std::to_string(error_code);
        }
    }
}
