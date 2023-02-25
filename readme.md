# Usage

The temperature can be viewed as follows:

    user@pi:~ temp
    69.294

## Commands

### help

Displays the available commands.

### log

Displays 15 previous temperature log entries.


# Installation

Install templogger using 

    make install

Then start the service using

    sudo systemctl enable templogger

# Uninstallation

Disable the service

    sudo systemctl disable templogger

Then do the uninstallation

    make uninstall