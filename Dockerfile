FROM golang:1.13-rc

RUN mkdir /plugin

WORKDIR /plugin

COPY . .

RUN go install

CMD [ "protoc-gen-twirp_swagger" ]