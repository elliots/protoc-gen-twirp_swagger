#!/bin/bash

for l in javascript java python ruby lua csharp html2; do 
  docker run --rm -v "$(pwd):/work" swaggerapi/swagger-codegen-cli generate \
    -i "/work/service.swagger.json" \
    -l "$l" \
    -o "/work/clients/$l"
done

echo "See https://github.com/swagger-api/swagger-codegen for more info"