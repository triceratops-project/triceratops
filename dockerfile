FROM alpine

WORKDIR /opt/triceratops

RUN apk update
RUN apk add ca-certificates --no-cache 
RUN apk add openssl --no-cache 

COPY ./target/release/triceratops-server triceratops
COPY ./apps/panel/build ./apps/panel/build

ENTRYPOINT [ "./triceratops", "start" ]