# building container 
FROM registry.fedoraproject.org/fedora-minimal AS build 
RUN mkdir /go && microdnf install golang openssl && microdnf clean all 
WORKDIR /go 
RUN export GOPATH=/go; CGO_ENABLED=0 go get github.com/peterducai/remath && go build github.com/peterducai/remath
COPY generate_certs.sh .
RUN chmod +x generate_certs.sh && ./generate_certs.sh

FROM registry.fedoraproject.org/fedora-minimal 
WORKDIR / 
COPY static /static
RUN ls -la / && ls -la /usr/local/bin 
COPY --from=build /go/bin/remath /usr/local/bin 
COPY --from=build /go/server.* /
CMD ["remath"] 