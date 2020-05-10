FROM rust:1.41
WORKDIR /usr/src/doraemon

COPY . .

RUN cargo build --release

#----------------------------------------

FROM rust:1.41
WORKDIR /usr/app

COPY --from=0 /usr/src/doraemon/target/release/doraemon doraemon

# Templates
RUN ["mkdir", "-p", "src", "templates"]
COPY src/templates src/templates

# Resizer data
RUN ["mkdir", "-p", "data/resizer"]

EXPOSE 8000
CMD ["./doraemon"]
