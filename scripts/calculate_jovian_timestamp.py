# Usage:
# FORK_DATE="2026-03-17T12:00:00" DISPUTE_FACTORY_ADDRESS="" L1_URL="" L2_URL="" python3 ./scripts/calculate_jovian_timestamp.py
#
# Description:
# This script calculates the Jovian timestamp by finding the next L2 block of the first game's l2BlockNumber after the given fork date.
# See https://github.com/celo-org/celo-blockchain-planning/issues/1327#issuecomment-3951692589 for the reasoning.


from datetime import datetime, timezone
import subprocess, os

dt = datetime.fromisoformat(os.environ["FORK_DATE"]).replace(tzinfo=timezone.utc)
fork_time_min = int(dt.timestamp())
def run(cmd):
    return subprocess.check_output(cmd, shell=True).decode().strip()

DISPUTE_FACTORY_ADDRESS = os.environ["DISPUTE_FACTORY_ADDRESS"]
L1_URL = os.environ["L1_URL"]
L2_URL = os.environ["L2_URL"]

game_count = int(run(f'cast to-dec $(cast call {DISPUTE_FACTORY_ADDRESS} "gameCount()" -r {L1_URL})'))
game_index = game_count - 1
game_addr = run(f'cast call {DISPUTE_FACTORY_ADDRESS} "gameAtIndex(uint256)(uint32,uint64,address)" {game_index} -r {L1_URL}').splitlines()[-1].strip()
l2_block = int(run(f'cast to-dec $(cast call {game_addr} "l2BlockNumber()" -r {L1_URL})'))
current_game_l2_time = int(run(f'cast block {l2_block} -f timestamp -r {L2_URL}'))

next_timestamp = current_game_l2_time
while next_timestamp < fork_time_min:
    next_timestamp += 1800

fork_timestamp = next_timestamp + 1
print('Jovian timestamp: ', fork_timestamp)
print('Jovian date: ', datetime.fromtimestamp(fork_timestamp, timezone.utc))
