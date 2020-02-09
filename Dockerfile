FROM rust:1.41
RUN echo $PATH

WORKDIR /usr/src/doraemon
RUN ["mkdir", "-p", "data/resizer"]

COPY . .

RUN cargo build --release
RUN mv target/release/doraemon doraemon

EXPOSE 8000
CMD ["./doraemon"]
