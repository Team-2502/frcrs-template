git -v

mkdir homebrew && curl -L https://github.com/Homebrew/brew/tarball/master | tar xz --strip 1 -C homebrew

eval "$($HOME/homebrew/bin/brew shellenv)"

brew install openjdk@17

export JAVA_HOME=$HOME/homebrew/Cellar/openjdk@17/17.0.11/

curl https://sh.rustup.rs -sSf | sh -s -- -y

rustup target add arm-unknown-linux-gnueabi

git clone https://github.com/wpilibsuite/allwpilib.git /var/tmp/
cd /var/tmp/allwpipib && ./gradlew installRoborioToolchain
rm -rf /var/tmp/allwpilib

echo "export PATH=$HOME/.gradle/toolchains/frc/2024/roborio/bin:'$PATH'" >> .zshrc

#curl "https://code.visualstudio.com/sha/download?build=stable&os=darwin-arm64" -o vscode.zip && unzip vscode.zip