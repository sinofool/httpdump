FROM golang:1.17 AS build
WORKDIR /code
ADD main.go /code/
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o server main.go

FROM scratch
COPY --from=build /code/server /server
ENTRYPOINT ["/server"]

