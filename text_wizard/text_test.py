import text_wizard
import time

# একটি বিশাল টেক্সট ডাটা
big_text = "rust is fast python is easy rust and python together are powerful " * 100000

start_time = time.time()

# Rust-এ লেখা ফাংশন কল করা
unique_count = text_wizard.count_unique_words(big_text)

print(f"Unique words found: {unique_count}")
print(f"Time taken by Rust: {time.time() - start_time} seconds")
