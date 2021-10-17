package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	log.Fatal(http.ListenAndServe(":8080", &HttpDump{}))
}

type HttpDump struct {
}

func (t *HttpDump) ServeHTTP(writer http.ResponseWriter, request *http.Request) {
	msg := fmt.Sprintf("%s %s\n", request.Method, request.RequestURI)
	for key, value := range request.Header {
		msg += fmt.Sprintf("%s: %s\n", key, value)
	}
	msg += "\n"
	log.Printf(msg)
	writer.WriteHeader(200)
}
