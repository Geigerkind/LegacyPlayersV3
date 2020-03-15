FROM archlinux

RUN pacman -Sy \
    && pacman -S --noconfirm git rustup clang openssl pkg-config \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup component add clippy;

COPY start.sh /

CMD ["/bin/bash", "/start.sh"]