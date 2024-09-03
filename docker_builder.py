import subprocess

def run_command(command):
    """Run a shell command and print the output."""
    print(f"Running: {command}")
    result = subprocess.run(command, shell=True, text=True)
    if result.returncode != 0:
        print(f"Command failed with return code {result.returncode}")
        exit(result.returncode)

def build_with_cache(target, image_name, cache_from=None, no_cache=False):
    """Build a Docker image with optional caching."""
    cache_option = "--no-cache" if no_cache else ""
    cache_from_option = f"--cache-from {cache_from}" if cache_from else ""
    command = f"docker build {cache_option} {cache_from_option} --target {target} -t {image_name} ."
    run_command(command)

def main():
    # Build Rust dependencies stage with caching
    build_with_cache(target="base-deps", image_name="rchat:base-deps")

    # Build Rust project using cache from base-deps
    build_with_cache(target="project-builder", image_name="rchat:project-builder", cache_from="rchat:base-deps")

    # Build Node.js dependencies with caching
    build_with_cache(target="dependencies", image_name="rchat:dependencies")

    # Build frontend projects using cache from dependencies
    build_with_cache(target="build", image_name="rchat:build", cache_from="rchat:dependencies")

    # Build final runtime image without cache
    build_with_cache(target="runner", image_name="rchat:runner", cache_from="rchat:project-builder", no_cache=True)

if __name__ == "__main__":
    main()