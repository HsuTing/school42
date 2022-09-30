FROM ghcr.io/hsuting/code-server:main as base

# config
COPY ./root /root

# build image
FROM alpine as build

# install packages
RUN apk add \
  jq

# merge settings.json
COPY --from=base /root/.local/share/code-server/User/settings.json ./base-settings.json
COPY --from=base /root/.local/share/code-server/User/school42-settings.json ./school42-settings.json
RUN jq '. + input' ./base-settings.json ./school42-settings.json > ./settings.json

# main image
FROM base

# code-server
RUN code-server --install-extension kube.42header

# config
COPY --from=build /root/.settings.json /root/.local/share/code-server/User/settings.json
RUN rm /root/.local/share/code-server/User/school42-settings.json
