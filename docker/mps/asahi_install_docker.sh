#!/bin/sh
# SPDX-License-Identifier: MIT

set -e

if [ "${0%/*}" != "$0" ]; then
    cd "${0%/*}"
fi

export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

export DYLD_LIBRARY_PATH=$PWD/Frameworks/Python.framework/Versions/Current/lib
export DYLD_FRAMEWORK_PATH=$PWD/Frameworks
python=Frameworks/Python.framework/Versions/3.9/bin/python3.9
export SSL_CERT_FILE=$PWD/Frameworks/Python.framework/Versions/Current/etc/openssl/cert.pem
# Bootstrap does part of this, but install.sh can be run standalone
# so do it again for good measure.
export PATH="$PWD/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"


if [ $(arch) != "arm64" ]; then
    echo
    echo "You're running the installer in Intel mode under Rosetta!"
    echo "Don't worry, we can fix that for you. Switching to ARM64 mode..."

    # This loses env vars in some security states, so just re-launch ourselves
    exec arch -arm64 ./install.sh
fi

exec </dev/tty >/dev/tty 2>/dev/tty
exec $python main.py "$@"
