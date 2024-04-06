#!/bin/bash
set -ex

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

PATH="/tmp/cargo/bin:${PATH}"
export PATH

# Compile wheels
for PYBIN in \
    /opt/python/cp37*/bin
do
    rm -rf /io/build/
    "${PYBIN}/pip" install -U setuptools setuptools-scm setuptools-rust wheel
    "${PYBIN}/pip" wheel /io/ -w /io/dist/ --no-deps
done

# Bundle external shared libraries into the wheels
for whl in /io/dist/*cp3*.whl
do
    auditwheel repair "$whl" -w /io/dist/
done

# Install packages and test
for PYBIN in \
    /opt/python/cp37*/bin
do
    "${PYBIN}/pip" install mutuple -f /io/dist/
done
