#!/bin/bash

for l in go javascript java python ruby lua csharp; do 
  docker run --rm -v "$(pwd):/work" swaggerapi/swagger-codegen-cli generate \
    -i "/work/service.swagger.json" \
    -l "$l" \
    -o "/work/clients/$l"
done

docker run --rm -v "$(pwd):/work" swaggerapi/swagger-codegen-cli langs

echo "See https://github.com/swagger-api/swagger-codegen for more info"