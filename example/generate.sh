#!/bin/bash

go get github.com/elliots/protoc-gen-twirp_swagger

protoc --go_out=. \
       --twirp_out=. \
       --twirp_swagger_out=. \
       ./service.proto