LIB=robotcode
OUT=target/arm-unknown-linux-gnueabi/release/$(LIB)
TEAM=25.02

$(OUT): src/** Cargo.*
	cargo build --release --target arm-unknown-linux-gnueabi

.PHONY: deploy-scp
deploy-scp: $(OUT)
	ssh lvuser@10.$(TEAM).2 /usr/local/frc/bin/frcKillRobot.sh
	ssh lvuser@10.$(TEAM).2 rm $(LIB)
	scp $(OUT) lvuser@10.$(TEAM).2:
	ssh lvuser@10.$(TEAM).2 /usr/local/frc/bin/frcRunRobot.sh