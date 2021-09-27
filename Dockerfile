FROM rust:latest
COPY ./ ./
RUN cargo build --release
EXPOSE 5000
CMD ["./target/release/cv_api"]