export CALLDWELL_BOARD_LOGIN=$AERUGO_BOARD_LOGIN
export CALLDWELL_BOARD_PASSWORD=$AERUGO_BOARD_PASSWORD
export CALLDWELL_BOARD_HOSTNAME=$AERUGO_BOARD_HOSTNAME
export CALLDWELL_BOARD_GDB_PORT=$AERUGO_BOARD_GDB_PORT
export CALLDWELL_BOARD_RTT_PORT=$AERUGO_BOARD_RTT_PORT
export CALLDWELL_GDB_EXECUTABLE="arm-none-eabi-gdb"
export CALLDWELL_TARGET_TRIPLE="thumbv7em-none-eabihf"
export CALLDWELL_BOARD_GDB_EXEC_SCRIPT="./setup_debugging_sam_clean.sh"

poetry run python ./examples/$1
