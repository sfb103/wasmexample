#!/bin/bash
cmake -GNinja . -DCMAKE_MAKE_PROGRAM=/usr/bin/ninja
cmake --build . --target infoxchange.wasm -- -j8