import os
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

# Define the binary command
binary_command = "target/debug/filter_example"

# Set the number of times to run each host
n = 10

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

# Plot the chart
plt.bar(hosts, errors)
plt.xlabel("Host")
plt.ylabel("Error Count")
plt.title("GetCookiesError Occurrences by Host")
plt.gca().yaxis.set_major_locator(MultipleLocator(1))  # Set y-axis tick frequency to 1

# Create the "results" directory if it doesn't exist
os.makedirs("results", exist_ok=True)

# Save the chart as a JPG file
output_file = os.path.join("results", "chart.jpg")
plt.savefig(output_file, dpi=300, bbox_inches="tight")
print(f"Chart saved to {output_file}")
