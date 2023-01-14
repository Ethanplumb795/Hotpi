import ctypes

rust = ctypes.CDLL("hotpid/target/release/librust_lib.so")

if __name__ == "__main__":
    SOME_BYTES = "Hello, World!".encode("utf-8")
    rust.print_string(SOME_BYTES)
