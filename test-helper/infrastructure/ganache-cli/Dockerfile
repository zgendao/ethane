# node:alpine will be our base image to create this image
FROM node:alpine

ARG WEB3_FORK_URL
ENV WEB3_FORK_URL ${WEB3_FORK_URL}

# Set the /app directory as working directory
WORKDIR /app

# Install ganache-cli globally
RUN npm install -g ganache-cli

EXPOSE 8545

# Set the default command for the image
# Add a fix private key and 1000 ETH on it.
CMD ganache-cli -h 0.0.0.0 --fork ${WEB3_FORK_URL} --account "0x31c354f57fc542eba2c56699286723e94f7bd02a4891a0a7f68566c2a2df6795,1000000000000000000000" --account "0x31c354f57fc342eba2c56699286723e94f7bd12a4891a0a7f68566c2a2df6795,1000000000000000000000"