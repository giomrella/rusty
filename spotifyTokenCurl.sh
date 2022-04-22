#!/bin/bash
curl https://accounts.spotify.com/api/token \
	-H "Authorization: Basic ${SPOTIFY_CLIENT_CREDS_B64}" \
	-H "Content-Type: application/x-www-form-urlencoded" \
	-X POST -d "grant_type=client_credentials"
#curl https://accounts.spotify.com/api/token \
#	-H "Authorization: Basic ${SPOTIFY_CLIENT_CREDS_B64}" \
#	-H "Content-Type: application/json" \
#	-X POST -d '{"grantType": "client_credentials"}'
