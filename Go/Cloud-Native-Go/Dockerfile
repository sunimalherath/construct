FROM golang:1.24.3-alpine3.20

MAINTAINER Sunimal Herath

ENV SOURCES /home/sunio/Developer/Projects/construct/Go/Cloud-Native-Go/

COPY . ${SOURCES}

RUN cd ${SOURCES} && CGO_ENABLED=0 go install

ENV PORT 8080
EXPOSE 8080

ENTRYPOINT cloud-native-go
