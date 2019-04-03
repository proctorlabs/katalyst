FROM golang

RUN go get github.com/parkghost/gohttpbench
RUN go build -o gb github.com/parkghost/gohttpbench

ENTRYPOINT [ "./gb" ]