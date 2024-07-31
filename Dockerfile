# Use the official manylinux2014 image
#FROM quay.io/pypa/manylinux2014_x86_64
FROM quay.io/pypa/manylinux_2_28_aarch64

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


RUN /opt/python/cp37-cp37m/bin/pip install maturin && \
    /opt/python/cp38-cp38/bin/pip install maturin && \
    /opt/python/cp39-cp39/bin/pip install maturin && \
    /opt/python/cp310-cp310/bin/pip install maturin && \
    /opt/python/cp311-cp311/bin/pip install maturin

# Set the working directory
WORKDIR /io

# Copy the project files
COPY . .

# Build the wheels
RUN /opt/python/cp37-cp37m/bin/maturin build --release && \
    /opt/python/cp38-cp38/bin/maturin build --release && \
    /opt/python/cp39-cp39/bin/maturin build --release && \
    /opt/python/cp310-cp310/bin/maturin build --release && \
    /opt/python/cp311-cp311/bin/maturin build --release
