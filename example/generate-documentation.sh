#!/bin/bash

docker run --rm -v "$(pwd):/work" swaggerapi/swagger-codegen-cli generate \
    -i "/work/service.swagger.json" \
    -l "html2" \
    -o "/work/documentation/codegen-html2"

docker run --rm -v "$(pwd):/work" sourcey/spectacle spectacle \
    -t "/work/documentation/spectacle" \
    "/work/service.swagger.json"

