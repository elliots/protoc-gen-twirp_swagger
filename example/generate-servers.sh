#!/bin/bash

for l in nodejs-server rust-server ; do 
  docker run --rm -v "$(pwd):/work" swaggerapi/swagger-codegen-cli generate \
    -i "/work/service.swagger.json" \
    -l "$l" \
    -o "/work/servers/$l"
done

docker run --rm -v "$(pwd):/work" swaggerapi/swagger-codegen-cli langs

echo "See https://github.com/swagger-api/swagger-codegen for more info"
