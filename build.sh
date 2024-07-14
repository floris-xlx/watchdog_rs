# go to the deploy directory
mkdir -p ~/deploys && cd ~/deploys || exit

# delete old repo if exists
rm -rf ~/deploys/{repo_name} || exit

# clone the repo
git clone --depth 1 {{repo_url}} ./{repo_name} || exit

# go to the repo
cd ./{repo_name} || exit

# run tests
TEST_OUTPUT=$(cargo test)
echo "$TEST_OUTPUT"

# check if test output contains "test result: ok"
if ! echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
  echo "Tests failed"
  exit 1
fi

# build the project
if ! cargo build --release; then
  echo "Build failed"
  exit 1
fi

# stop the old service
pkill -f ./{service_name} || true

# copy the binary to the service directory
cp target/release/{service_name} /{service_name} || exit

# start the service
nohup ./{service_name} &

# check if the service is running
if ! pgrep -f ./{service_name}; then
  echo "Service failed to start"
  exit 1
fi

echo "Deployed successfully"

# go back to the previous directory
cd - || exit