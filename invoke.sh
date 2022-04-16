#!/bin/bash
#cargo lambda invoke --data-ascii '{"firstName": "gio"}' rusty
cargo lambda invoke --data-ascii '{ "event": { "channel": "jeremys_in_a_sandbox", "user": "UDLCBR4F6", "authorizations": [{"user_id": "U01UTH2J666"}], "text": "Hey, Rusty!" } }' rusty
