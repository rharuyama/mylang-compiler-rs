#!/bin/bash

docker build -t ubuntu
docker run -it -d --name mylang-compiler-rs -v $(pwd):/home/
