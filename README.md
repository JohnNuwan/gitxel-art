# Gitxel-Art
                                        _ _            _              _    
                                       (_) |          | |            | |   
                                   __ _ _| |___  _____| |   __ _ _ __| |_  
                                  / _` | | __\ \/ / _ \ |  / _` | '__| __| 
                                 | (_| | | |_ >  <  __/ | | (_| | |  | |_  
                                  \__, |_|\__/_/\_\___|_|  \__,_|_|   \__| 
                                   __/ | 

                                              ##          ##
                                                ##      ##        
                                              ##############
                                            ####  ######  ####
                                          ######################
                                          ##  ##############  ##     
                                          ##  ##          ##  ##
                                                ####  ####

## Introduction

**Gitxel-Art** is a fun and creative way to add pixel art to your GitHub contributions graph! 
By generating a series of commits, you can create images on your GitHub profile directly in the contributions graph. 
Whether you’re looking to display your artistic skills or just have a little fun with GitHub's contribution calendar, 
Gitxel-Art allows you to do just that.

This project uses a *grid-based* approach to map pixel art into your contribution graph,
where each "pixel" corresponds to one or more commit on a specific day of the year. 
The result is a colorful and unique visual representation of your contributions. 
You can either generate your own pixel art or use the default Space Invader image provided for testing.

This tool is under [MIT license](LICENSE.md).

## Disclaimer

**Gitxel-Art** is intended solely for educational and entertainment purposes. 
The creator of this project disclaims any responsibility for the use of this tool, and by using it, 
you agree that any consequences resulting from the generation of commits,
including but not limited to unwanted repository activity or accidental exposure of data, are solely your responsibility.

This tool is not endorsed by GitHub, and it is recommended that you use it only for fun or learning. 
Please ensure that any repositories used are private, and do not use this tool in ways that violate GitHub's Terms of Service.

**Use this tool at your own risk. The creator is not liable for any damages or unexpected outcomes related to the use of this project.**


Before:
![github_before](media/github_before.png)

After:
![github_before](media/github_after.png)


## Installation & Build (Rust)

Gitxel-Art is rewritten in **Rust** for maximum speed, zero external runtime dependencies (no Python or Matplotlib required), and a rich terminal experience.

### Prerequisites

Ensure you have Rust and Cargo installed:
- [Install Rust](https://rustup.rs/)

### Build the project

```shell
git clone <url-repo>
cd gitxel-art
cargo build --release
```

The compiled standalone binary will be available at `./target/release/gitxel-art` (or `gitxel-art.exe` on Windows).

## Usage

### Interactive Wizard Mode

Simply launch the binary without arguments to enter the interactive setup:

```shell
cargo run --release
# or directly:
./target/release/gitxel-art
```

The wizard will guide you through:
1. Selecting a pixel art template (from `./pixel_art/` or a custom image)
2. Specifying the target local Git repository
3. Setting the dummy file name (default: `dummy.txt`)
4. Choosing the target year (e.g. `2024`)
5. Choosing your action: preview in terminal, generate commits locally, or dry-run simulation

### CLI Arguments (Fast & CI/CD friendly)

You can also run non-interactively with CLI flags:

```shell
# Terminal TrueColor preview only (no commits created)
./target/release/gitxel-art --image ./pixel_art/Space_invader.jpg --preview

# Simulate commit generation with a progress bar without touching any files
./target/release/gitxel-art --image ./pixel_art/pac_man.png --repo /path/to/dummy_repo --year 2024 --dry-run

# Generate commits on a local repository
./target/release/gitxel-art --image ./pixel_art/pokemon.png --repo /path/to/dummy_repo --year 2024

# Automatically push to origin/main after generation
./target/release/gitxel-art --image ./pixel_art/pokemon.png --repo /path/to/dummy_repo --year 2024 --push
```

### CLI Options

| Flag | Description | Default |
|------|-------------|---------|
| `-i, --image <FILE>` | Path to image (PNG/JPEG, max 49x7) | `./pixel_art/Space_invader.jpg` |
| `-r, --repo <DIR>` | Target local Git repository | Current directory |
| `-y, --year <YEAR>` | Target contribution year ($\ge 1974$) | `2024` |
| `-d, --dummy-file <NAME>` | Name of dummy file | `dummy.txt` |
| `-p, --preview` | Preview TrueColor calendar in terminal | `false` |
| `--dry-run` | Simulate commit creation without writing | `false` |
| `--push` | Push to `origin/main` automatically | `false` |
| `--interactive` | Force interactive prompt wizard | `false` |


Then, you will see your beautiful artwork displayed on your GitHub profile:

![github_after](media/github_after.png)

## Removing Your Pixel Art from GitHub

To remove your pixel art from your GitHub profile, simply delete the repository you created for generating the commits. 
*For example*, if your repository is named `dummy_repo, deleting it will remove the commits from your profile:

![remove](media/remove_art.png)

## Notes

- You can run the script as many times as you'd like, with different images and configurations.
- The script will always generate a new set of commits for each image and year you specify.
- Make sure your repository remains private unless you want others to see your dummy_file.
- If your repository is private, make sure to activate the `private contribution` on your contribution graph's setting:

![github_contribution_setting](media/contribution_setting.png)

## Acknowledgements

Gitxel-Art is a fun way to create pixel art in your GitHub contributions graph, 
and it's inspired by various pixel art projects on GitHub. 
Feel free to contribute or suggest improvements to the project!





