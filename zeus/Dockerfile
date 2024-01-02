FROM rust:latest
WORKDIR /app
COPY . .
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN echo "yes" | cargo binstall cargo-watch
RUN echo "yes" |  cargo binstall sqlx-cli
CMD ["cargo", "watch", "-x", "run"]