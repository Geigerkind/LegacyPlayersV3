export CHARACTERS_URL=mysql://root:vagrant@172.17.0.1/characters
export ROCKET_DATABASES="{characters={url=\"${CHARACTERS_URL}\"}, lp_consent={url=\"${LP_CONSENT_URL}\"}}"
export LP_API_TOKEN=4c3bdeddb0b9d054831866a2061f4a6a8e100633610f5cd4f4a59638f60d8fb359f68af84d5410748c55545f78b5030bcd8ad33b7e8861a373aadde37315f222
export URL_AUTHORIZATION_ENDPOINT=http://localhost:8001/token_validator
export CHARACTER_FETCH_INTERVAL_IN_SEC=60
export EXPANSION_ID=2
export UID_SALT=SomeSalt
export OPT_IN_MODE=false