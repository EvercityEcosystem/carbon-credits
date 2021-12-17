FROM paritytech/ci-linux:production as builder

ARG PROFILE=release
WORKDIR /home/source

RUN git clone -b v3.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git /home/source
COPY ./node_override ./
# COPY ./ ./pallets/filesign
RUN cargo update -p parity-db
RUN cargo update -p wasm-bindgen
RUN cargo test && cargo build --$PROFILE


FROM debian:buster-slim

LABEL org.label-schema.vendor="Evercity" \
      org.label-schema.name="Filesign Substrate Template Node" \
      org.label-schema.description="Filesign Substrate Template Node" \
      org.label-schema.url="https://evercity.io" \
      org.label-schema.schema-version="1.0" \
      org.opencontainers.image.source="https://github.com/EvercityEcosystem/filesign"

ARG PROFILE=release
COPY --from=builder /home/source/target/$PROFILE/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node node && \
	mkdir -p /.local/share && \
	mkdir /data && \
	chown -R node:node /data && \
	ln -s /.local/share/node-template && \
	rm -rf /usr/bin /usr/sbin

USER node
VOLUME ["/data"]

EXPOSE 30333 9933 9944
CMD ["node-template", "--dev", "--tmp", "--ws-external", "--rpc-external", "--rpc-cors", "all", "--port", "30300", "--rpc-port", "9933", "--ws-port", "9944"]
