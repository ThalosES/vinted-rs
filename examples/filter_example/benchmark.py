"""
CookieErrors Benchmark Script

This script benchmarks the amount of CookieErrors obtained when running the main.rs example.

The script runs the main.rs example multiple times for each host, capturing the number of
GetCookiesError occurrences. It then generates a bar chart to visualize the error counts.

*Discalimer:* Because this file is intended for internal debug, it is certainly not the most
efficient implementation and it is not tested at all

"""
import os, sys
import subprocess
from enum import Enum
import matplotlib.pyplot as plt
from matplotlib.ticker import MultipleLocator
from tqdm import tqdm

class Host(Enum):
    Fr = "fr"
    Be = "be"
    Es = "es"
    Lu = "lu"
    Nl = "nl"
    Lt = "lt"
    De = "de"
    At = "at"
    It = "it"
    Uk = "co.uk"
    Pt = "pt"
    Com = "com"
    Cz = "cz"
    Sk = "sk"
    Pl = "pl"
    Se = "se"
    Ro = "ro"
    Hu = "hu"

# Check the number of command-line arguments
if len(sys.argv) != 2:
    print("Invalid number of arguments. Usage: python benchmark.py <n>")
    sys.exit(1)

n = int(sys.argv[1])

# Define the binary command
binary_command = "target/debug/filter_example"

# Initialize a dictionary to store the error counts
ok_counts = {}

# Create the progress bar
progress_bar = tqdm(Host, desc="Processing", unit="host")

# Run the binary for each host and capture the error counts
for host in progress_bar:
    ok_count = n
    for _ in range(n):
        process = subprocess.run(
            [binary_command, host.value],
            capture_output=True,
            text=True
        )
        output = process.stderr.strip()
        ok_count -= output.count("GetCookiesError")

    ok_counts[host.value] = ok_count
    progress_bar.set_postfix({"Host": host.value})

# Close the progress bar
progress_bar.close()

# Prepare the data for plotting
hosts = list(ok_counts.keys())
errors = list(ok_counts.values())

# Set the style and color palette
colors = plt.cm.Set3(range(len(hosts)))

# Create a figure with a larger size
fig, ax = plt.subplots(figsize=(10, 6))

# Plot the chart
bars = ax.bar(hosts, errors, color=colors)

# Customize the plot
plt.xlabel("Host", fontsize=12)
plt.ylabel("Error Count", fontsize=12)
plt.title("200-OK status code received", fontsize=14)
ax.yaxis.set_major_locator(MultipleLocator(1))  # Set y-axis tick frequency to 1
ax.grid(axis="y", linestyle="--", alpha=0.5)

# Add data labels to the bars
for bar in bars:
    height = bar.get_height()
    ax.annotate(f"{height}", xy=(bar.get_x() + bar.get_width() / 2, height),
                xytext=(0, 3), textcoords="offset points",
                ha="center", va="bottom")

# Create the "results" directory if it doesn't exist
os.makedirs("results", exist_ok=True)

# Save the chart as a JPG file
output_file = os.path.join("results", "chart.jpg")
plt.savefig(output_file, dpi=300, bbox_inches="tight")
print(f"Chart saved to {output_file}")
