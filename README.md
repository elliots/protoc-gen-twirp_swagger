# protoc-gen-twirp_swagger

NOTE: WORK IN PROGRESS. COMPLETELY UNTESTED.


## What this is

A plugin for the awesome twirp - https://github.com/twitchtv/twirp.

It is a protobuf generator that creates a swagger file for your twirp services.

This swagger file can then be used to generate clients for many many languages.


## Usage

`go get github.com/elliots/protoc-gen-twirp_swagger`

then run as part of your usual protoc generation phase


```bash
protoc --go_out=. \
       --twirp_out=. \
       --twirp_swagger_out=. \
       ./test.proto
```

Check out [./_example/generate.sh](./example/generate.sh)


## Generating Clients

See: [./_example/generate-clients.sh](./_example/generate-clients.sh) for example clients for javascript, java, python, ruby, lua, and c#. As well as generated html documentation.

Note that there are a thousand options to pass to the generators, I'm just running the default.


## Thanks

Based on: https://github.com/grpc-ecosystem/grpc-gateway/tree/master/protoc-gen-swagger

Like, 99.5% based on.



## Example output

```json

{
  "swagger": "2.0",
  "info": {
    "title": "service.proto",
    "version": "version not set"
  },
  "schemes": [
    "http",
    "https"
  ],
  "consumes": [
    "application/json"
  ],
  "produces": [
    "application/json"
  ],
  "paths": {
    "/twirp/twitch.twirp.example.Haberdasher/MakeHat": {
      "post": {
        "summary": "MakeHat produces a hat of mysterious, randomly-selected color!",
        "operationId": "MakeHat",
        "responses": {
          "200": {
            "description": "",
            "schema": {
              "$ref": "#/definitions/exampleHat"
            }
          }
        },
        "parameters": [
          {
            "name": "body",
            "in": "body",
            "required": true,
            "schema": {
              "$ref": "#/definitions/exampleSize"
            }
          }
        ],
        "tags": [
          "Haberdasher"
        ]
      }
    }
  },
  "definitions": {
    "exampleHat": {
      "type": "object",
      "properties": {
        "size": {
          "type": "integer",
          "format": "int32",
          "description": "The size of a hat should always be in inches."
        },
        "color": {
          "type": "string",
          "description": "The color of a hat will never be 'invisible', but other than\nthat, anything is fair game."
        },
        "name": {
          "type": "string",
          "description": "The name of a hat is it's type. Like, 'bowler', or something."
        }
      },
      "description": "A Hat is a piece of headwear made by a Haberdasher."
    },
    "exampleSize": {
      "type": "object",
      "properties": {
        "inches": {
          "type": "integer",
          "format": "int32"
        }
      },
      "description": "Size is passed when requesting a new hat to be made. It's always\nmeasured in inches."
    }
  }
}

```