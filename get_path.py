import subprocess

url_or_path = "server/a.txt"
proc = subprocess.run(["cargo", "run", url_or_path], text=True, stdout=subprocess.PIPE)

file_path = proc.stdout.splitlines()[2][len("Written to ") :]
print(file_path)
