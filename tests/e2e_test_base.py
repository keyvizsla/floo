# DISCLAIMER: Do not run this script on your system.
# Should you choose to ignore this warning, your system may
# suffer from a broken floo configuration and more other side effects.
# This script is inteded to be ran from within a container of one of
# the docker images defined in this repository.
# Plese refer to the section on testing on https://keyvizsla.github.io/floo.

# Copyright (C) 2026 Leon Degel-Koehn
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

import os
import shutil
import uuid
import pexpect
import sys
import logging
from time import sleep

"""
Implements an E2E integration test for floo on bash.
This tests for the basic E2E functionality for floo.
The app is ran and a dummy fireplace is created and
selected. The test asserts that after this interaction,
which corresponds to the most basic usecase of floo,
the working directory of the shell has moved to the
directory of the selected fireplace.

Usage from repo root:
```sh
docker run --rm -v $(pwd):/app ghcr.io/keyvizsla/floo-linux:latest bash -c " \
  cargo install --path .; \
  python tests/e2e_test_base.py;
"
```
"""

logger = logging.getLogger(__name__)
prompt = "PROMPTSEP"


def create_dummy_dir() -> str:
    unique_id = str(uuid.uuid4())
    target_dir = os.path.join("/tmp", f"floo-test-{unique_id}")
    logger.info("Creating isolated test directory: %s", target_dir)
    os.makedirs(target_dir, exist_ok=True)
    return target_dir


def inject_floo_init(child):
    logger.info("Initializing floo wrapper via eval...")
    child.sendline('eval "$(floo-bin init)"')
    child.expect(prompt)


def spawn_shell():
    logger.info("Spawning bash shell...")
    child = pexpect.spawn("bash", echo=False,
                          encoding="utf-8", dimensions=(50, 120))
    child.sendline("stty -echo")
    child.sendline(f"export PS1='{prompt}'")

    child.expect(prompt)
    child.sendline("export TERM=xterm-256color")

    child.expect(prompt)
    return child


def create_and_select_fireplace(child, target_dir: str):
    assert os.path.isdir(target_dir)
    logger.info("Running: floo create '%s'", target_dir)
    child.sendline(f"floo create {target_dir}")
    child.expect("Create")
    logger.info("Naming dummy project 'a'...")
    child.send('a')
    sleep(0.5)
    child.send('\t')
    sleep(0.5)
    child.send('\t')
    sleep(0.5)
    child.send('\t')
    sleep(0.5)
    child.send('\r')
    sleep(0.5)
    child.expect("Select a Fireplace")
    logger.info("Selecting 'a' project")
    child.send('\r')
    child.expect(prompt)


def is_correct_pwd(child, target_dir) -> bool:
    logger.info("Verifying final shell directory...")
    child.sendline("pwd")
    child.expect(prompt)
    current_pwd = child.before

    logger.info("Expected %s", target_dir)
    logger.info("Actual %s", current_pwd)

    return current_pwd.strip() == target_dir.strip()


def run_e2e_test():
    try:
        target_dir = create_dummy_dir()
        child = spawn_shell()
        inject_floo_init(child)
        create_and_select_fireplace(child, target_dir)
        assert is_correct_pwd(child, target_dir)
        logger.info("Successfully ran")

    except pexpect.TIMEOUT:
        print("\n[-] FAILURE: Test timed out waiting for UI or prompt interaction.")
        print("Tail of child output for debugging:")
        print(child.before)
        sys.exit(1)

    except pexpect.EOF:
        print("\n[-] FAILURE: Bash shell exited prematurely.")
        sys.exit(1)

    finally:
        if os.path.exists(target_dir):
            logger.info("Cleaning up...")
            shutil.rmtree(target_dir)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    run_e2e_test()
