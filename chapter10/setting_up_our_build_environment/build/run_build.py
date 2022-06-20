from subprocess import Popen
from pathlib import Path
import json
import time


DIRECTORY_PATH = Path(__file__).resolve().parent

init_process = Popen(f"cd {DIRECTORY_PATH} && terraform init", shell=True)
init_process.wait()
apply_process = Popen(f"cd {DIRECTORY_PATH} && terraform apply", shell=True)
apply_process.wait()
produce_output = Popen(f"cd {DIRECTORY_PATH} && terraform output -json > {DIRECTORY_PATH}/output.json", shell=True)
produce_output.wait()

with open(f"{DIRECTORY_PATH}/output.json", "r") as file:
    data = json.loads(file.read())

server_ip = data["ec2_global_ips"]["value"][0][0]

print("waiting for sercer to be built")
time.sleep(5)
print("attempting to enter server")

build_process = Popen(f"cd {DIRECTORY_PATH} && sh ./run_build.sh {server_ip}", shell=True)
build_process.wait()


destroy_process = Popen(f"cd {DIRECTORY_PATH} && terraform destroy", shell=True)
destroy_process.wait()

