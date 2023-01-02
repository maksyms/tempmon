# TODO: make the deploy target more generic

set dotenv-load

alias d := deploy

build:
    cross build

localbuild:
    cargo build

release:
    cross build --release

localrelease:
    cargo build --release

# TODO: Parametrize the name of the program, source and target location
deploy: build
    scp /home/maksym/Work/tempmon/target/arm-unknown-linux-gnueabihf/debug/tempmon $DEPLOY_USERNAME@$DEPLOY_HOST:/home/$DEPLOY_USERNAME/bin/tempmon
    ssh $DEPLOY_USERNAME@$DEPLOY_HOST chmod +x /home/$DEPLOY_USERNAME/bin/tempmon
    scp /home/maksym/Work/tempmon/tempmon.service $DEPLOY_USERNAME@$DEPLOY_HOST:/home/$DEPLOY_USERNAME/tempmon.service
    scp /home/maksym/Work/tempmon/tempmon.toml $DEPLOY_USERNAME@$DEPLOY_HOST:/home/$DEPLOY_USERNAME/tempmon.toml

deployrelease: release
    scp /home/maksym/Work/tempmon/target/arm-unknown-linux-gnueabihf/release/tempmon $DEPLOY_USERNAME@$DEPLOY_HOST:/home/$DEPLOY_USERNAME/bin/tempmon
    ssh $DEPLOY_USERNAME@$DEPLOY_HOST chmod +x /home/$DEPLOY_USERNAME/bin/tempmon
    scp /home/maksym/Work/tempmon/tempmon.service $DEPLOY_USERNAME@$DEPLOY_HOST:/home/$DEPLOY_USERNAME/tempmon.service
    scp /home/maksym/Work/tempmon/tempmon.toml $DEPLOY_USERNAME@$DEPLOY_HOST:/home/$DEPLOY_USERNAME/tempmon.toml

run: deploy
    ssh $DEPLOY_USERNAME@$DEPLOY_HOST /home/$DEPLOY_USERNAME/bin/tempmon

clean:
    cargo clean
    rm -rf target
    ssh $DEPLOY_USERNAME@$DEPLOY_HOST rm -rf /home/$DEPLOY_USERNAME/bin/tempmon
