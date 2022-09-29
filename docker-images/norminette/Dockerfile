FROM ghcr.io/mikojs/base:main as base

FROM python:3.7-alpine as build

# norminette
RUN apk add git
RUN git clone https://github.com/42School/norminette.git
RUN cd norminette && \
  pip3 install -r requirements.txt && \
  python3 setup.py install
RUN rm -rf norminette

# bin
COPY --from=base /bin/entrypoint /bin/entrypoint

# run
ENTRYPOINT ["entrypoint", "norminette"]
