"""Smoke-test the installed wheel, not a binary from Cargo or PATH."""

import os
import subprocess
import sys
import sysconfig
import tempfile
from importlib.metadata import distribution
from pathlib import Path


def main():
    expected_python = sys.argv[1]
    expected_version = tuple(map(int, expected_python.removesuffix("t").split(".")))
    assert sys.version_info[:2] == expected_version, sys.version
    free_threaded = bool(sysconfig.get_config_var("Py_GIL_DISABLED"))
    assert free_threaded == expected_python.endswith("t"), sys.version
    if free_threaded:
        assert not sys._is_gil_enabled(), (
            "The free-threaded interpreter has its GIL enabled"
        )
    print(f"Testing Python {sys.version}; free-threaded={free_threaded}")

    package = distribution("lgtmeow")
    assert package.metadata["Requires-Python"] == ">=3.11"
    assert not package.requires, package.requires
    assert "Tag: py3-none-" in package.read_text("WHEEL")
    executable = Path(sysconfig.get_path("scripts")) / (
        "lgtmeow.exe" if os.name == "nt" else "lgtmeow"
    )
    assert executable.is_file(), executable

    # dirs::home_dir() uses HOME on Unix and USERPROFILE on Windows. Never
    # overwrite the caller's configuration, even when running this locally.
    with tempfile.TemporaryDirectory() as home:
        env = {**os.environ, "HOME": home, "USERPROFILE": home}

        def run(*args):
            result = subprocess.run(
                [str(executable), *args],
                env=env,
                cwd=home,
                stdin=subprocess.DEVNULL,
                capture_output=True,
                check=False,
                encoding="utf-8",
                timeout=30,
            )
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return result.stdout

        assert run("--version").strip() == f"lgtmeow {package.version}"
        assert "Usage:" in run("--help")
        run("setup", "--default")
        config_dir = Path(home) / ".config" / "lgtmeow"
        assert (config_dir / "config.toml").is_file()
        reply = run("-r")
        assert 'LGTMeow <img src="https://' in reply, reply
        assert 'width="14"' in reply, reply
        run("reset")
        assert not config_dir.exists()

    print(
        f"Installed lgtmeow {package.version}: wheel metadata and CLI smoke tests passed"
    )


if __name__ == "__main__":
    main()
