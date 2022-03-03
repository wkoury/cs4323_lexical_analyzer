FROM rust:latest

WORKDIR /myscanner
COPY . .

RUN cargo install --path .

CMD ["scanner", "samples/kim_example.ssc"]