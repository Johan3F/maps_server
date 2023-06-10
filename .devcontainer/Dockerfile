# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.224.2/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:1

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update

# GPG Signing commits from within the devcontainer
RUN apt-get install gnupg2 -y

USER vscode
# Installing diesel cli
cargo install diesel_cli --no-default-features --features postgres