FROM rust:1.41
WORKDIR /usr/src/doraemon

COPY . .

RUN cargo build --release

#----------------------------------------

FROM rust:1.41
WORKDIR /usr/app

COPY --from=0 /usr/src/doraemon/target/release/doraemon doraemon
RUN ["mkdir", "-p", "data/resizer"]

EXPOSE 8000
CMD ["./doraemon"]
