FROM kincirangin/backenv:latest

COPY . /app
WORKDIR /app

RUN cargo build --release

EXPOSE 50002

CMD ["target/release/auth"]