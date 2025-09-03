# locwork

A simple cli tool written for my own convenience.

## What is locwork?

Locwork is a simple command-line tool to help you track your work location. If you work from different locations (like home, the office, or different satellite offices), you can use `locwork` to log where you are working each day.

It also provides some fun statistics to see how your work locations are distributed over time.

## Interactive Mode

The primary way to use `locwork` is through its interactive mode. This provides a simple and quick interface for logging your daily work location.

You can start the interactive mode by running:
```shell
locwork -i
```

This will present you with a menu to log your location for the day, view stats, or quit.

*(Screenshots of interactive mode can be added here)*

### Launching at Startup

For convenience, you might want `locwork`'s interactive mode to launch automatically when you start your system. You will need to configure this yourself, as the method varies depending on your operating system.

## Command-Line Usage

For scripting and automation, you can use the command-line interface directly.


### Managing Locations

You need to add your work locations before you can log them.

- **Add a location:**
  ```shell
  locwork location add "My Office"
  ```

- **Remove a location:**
  ```shell
  locwork location remove "My Office"
  ```

- **List all locations:**
  ```shell
  locwork location list
  ```

### Logging Your Work

Once you have locations, you can log your work for specific days.

- **Log today's location:**
  ```shell
  locwork log add "My Office" -t
  ```

- **Log a location for a specific date:**
  ```shell
  locwork log add "Home" -d 2023-10-27
  ```

- **Mark a day as a holiday:**
  You can mark any day as a holiday. This is useful for tracking days off.
  ```shell
  locwork log add "Home" -d 2023-12-25 --holiday
  ```
  When logging a holiday, the location doesn't matter as much, but is still required.

## Installation

### Linux/Manjaro
You should be able to install directly from github, just run this command.
```shell
pipx install git+https://github.com/gvdbroek/locwork/
```


### Windows
For windows, it has to be installed to for the user.
Note that you also have to add the script's location in your `$PATH`
```shell
pip install git+https://github.com/gvdbroek/locwork/ --user
```
After that, you should be able to run the command as such.


## Updating

### Linux/Manjaro
The easiest way to update locwork is by just reinstalling the script using 
```shell
pip install --force git+https://github.com/gvdbroek/locwork/
``` 

### Windows
```shell
pip install --force git+https://github.com/gvdbroek/locwork/ --user

```

