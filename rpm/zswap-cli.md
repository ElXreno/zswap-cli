% zswap-cli(1)

# NAME

zswap-cli - Utility for controlling zswap parameters

# SYNOPSIS

**zswap-cli** [**OPTION**]...

# DESCRIPTION

zswap-cli is a simple utility for controlling zswap parameters easily. 

# OPTIONS

**-h**, **--help**
:   Prints help information

**-V**, **--version**
:   Prints version information

# SUBCOMMANDS

**help**
:   Prints this message or the help of the given subcommand(s)

**info**
:   Displays current parameters

**set**
:   Sets configuration

**stats**
:   Displays current zswap stats

# OPTIONS SUBCOMMAND set

**--use-config**
:   Use config file from /etc/zswap-cli.conf

# OPTIONS SUBCOMMAND stats

**-a**, **--all**
:   Displays all debug variables

# CONFIGURATION OPTIONS

zswap-cli support of getting options from configuration file (located in /etc/zswap-cli.conf).

# Supported options in configuration file

  * **ZSWAP_ENABLED** - [**YN**] enable or disable zswap kernel module.
  * **ZSWAP_SAME_FILLED_PAGES_ENABLED** [**YN**] - enable or disable memory deduplication.
  * **ZSWAP_ACCEPT_THREHSOLD_PERCENT** [**1-100**] - threshold at which zswap would start accepting pages again after it became full.
  * **ZSWAP_MAX_POOL_PERCENT** [**1-100**] - maximum percentage of memory that the compressed pool can occupy.
  * **ZSWAP_COMPRESSOR** [**zstd/lz4**] - default comression algorithm.
  * **ZSWAP_ZPOOL** [**z3fold/zbud**] - zpool type.

# SYSTEMD UNIT

After installation, the systemd-unit **zswap-cli.service** will be added.

## Changing configuration file

All configuration parameters are stored in **/etc/zswap-cli.conf** file.

## Enabling unit

Enable systemd-unit and run it on system startup:

```
sudo systemctl enable --now zswap-cli.service
```

## Disabling unit

Disable systemd-unit and stop runing it on system startup:

```
sudo systemctl disable zswap-cli.service
```

## Running unit

You can also run systemd-unit without adding it to startup.

Start unit and enable ZSwap:

```
sudo systemctl start zswap-cli.service
```

Stop unit and disable ZSwap:

```
sudo systemctl stop zswap-cli.service
```

# AUTHORS

ElXreno ⟨elxreno@gmail.com⟩.
