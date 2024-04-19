FROM apachepulsar/pulsar:3.2.2

# Expose required ports
EXPOSE 6650 8080

# Mount volumes
VOLUME /pulsar/data
VOLUME /pulsar/conf

# Set the working directory
WORKDIR /pulsar

# Command to run Pulsar standalone
CMD ["bin/pulsar", "standalone"]

