FROM rust:1.69.0
WORKDIR /var/app

# 4. Now that the dependency is built, copy your source code
COPY ./ ./

# 3. Build only the dependencies to cache them
RUN cargo build --release

EXPOSE 8000

CMD ["sh", "-c", "./target/release/${BINARY}"]