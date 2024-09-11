FROM rust:latest

RUN apt-get update && \
    apt install -y ca-certificates git default-jre gcc && \
    update-ca-certificates

#ENV GIT_SSL_NO_VERIFY=1

RUN ls /etc/ssl/certs && git config --global http.sslCAinfo /etc/ssl/certs/ca-certificates.crt && \
    git clone https://github.com/wpilibsuite/allwpilib.git
RUN cd allwpilib && ./gradlew installRoborioToolchain

#RUN update-ca-certificates

#RUN openssl req -new -newkey rsa:4096 -days 365 -nodes -x509 \
#    -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=cert" \
#    -keyout cert.key -out cert.cert

#RUN cp cert.cert /usr/local/share/ca-certificates
#RUN curl -k https://i.pki.goog/r1.crt -o ca-certificates.crt && cp ca-certificates.crt /etc/ssl/certs/
#RUN ls /usr/local/share/ca-certificates

#ENV GIT_SSL_NO_VERIFY=1
#RUN java -Djavax.net.ssl.trustStore=/usr/local/share/ca-certificates

#RUN git clone https://github.com/wpilibsuite/allwpilib.git
#RUN cd allwpilib && ./gradlew installRoborioToolchain



RUN rustup update && rustup default stable && rustup target add arm-unknown-linux-gnueabi