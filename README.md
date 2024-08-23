# frcrs-template
This is a template repository for FRC teams, designed to provide a starting point for projects using the frcrs framework. The template includes a basic structure to get you started with defining subsystems, configuring inputs, and handling commands in your robot code.

## Getting Started
### Prerequisites
Before using this template, make sure you have the following:
1. rust and the toolchain `arm-unknown-linux-gnueabi`
2. RoboRIO toolchain
```bash
git clone https://github.com/wpilibsuite/allwpilib
cd allwpilib
./gradlew installRoboRioToolchain
```
3. add to path `~/.gradle/toolchains/frc/2024/roborio/bin`
### Installation
Clone the repository:

```bash
git clone https://github.com/Team-2502/frcrs-template.git
cd frcrs-template
```
### Running
#### 1. Building robotcode
`cargo build --release --target arm-unknown-linux-gnueabi`
#### 2. copy javastub binary to RIO (only done once per rio)
download from `https://github.com/Team-2502/frcrs/releases`
or build from frcrs
#### 3. deploy code
1. use deploy from frcrs `https://github.com/Team-2502/frcrs/tree/jni/deploy`
   ```bash
   deploy --team-number TEAM --executable target/arm-unknown-linux-gnueabi/debug/robotcode
   ```
   If you need to copy javastub
   ```bash
   deploy --team-number TEAM --executable target/arm-unknown-linux-gnueabi/debug/robotcode --lib /path/to/javastub.jar
   ```
2. Manual
  ```bash
  scp target/arm-unknown-linux-gnueabi/debug/robotcode lvuser@roborio-TEAM-frc.local:.
  ```
  set /home/lvuser/robotCommand to `JAVA_HOME=/usr/local/frc/JRE /home/lvuser/robotcode`

### Usage
This template provides a basic setup for your robot code. Here’s how to get started:

#### 1. Configure the Robot
The `configure` function is where you should declare subsystems, joysticks, or any other components that need to be initialized on startup.

```rust
use frcrs::container;

pub struct Ferris;

pub async fn configure() {
    container!(container, Ferris {});
}
```
In this example, `Ferris` is a placeholder for your robot's struct. You can replace it with your actual struct that represents the robot or its subsystems.

#### 2. Define Commands
The `container` function is where you'll put your robot's commands. This function will be called in a loop, which by default is capped at 500Hz.

```rust
pub async fn container() {
    // Add your commands here
}
```
