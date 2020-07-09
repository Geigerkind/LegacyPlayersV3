export LP_CONSENT_URL=mysql://root:vagrant@lp_cm_mariadb/lp_consent
export CHARACTERS_URL=mysql://root:vagrant@172.17.0.1/characters
export ROCKET_DATABASES='{characters={url="${CHARACTERS_URL}"}, lp_consent={url="${LP_CONSENT_URL}"}}'
export LP_API_TOKEN=34f211e0ec4ca5ea67fe411b1e89f4e8bfe3d0155a3436bf85065b9c2209cb8ec6071b0c323076656250ce6caffd18bc6596967628ac375ca6c33161b4efbe35
export URL_AUTHORIZATION_ENDPOINT=http://localhost:8001/token_validator
export CHARACTER_FETCH_INTERVAL_IN_SEC=60
export EXPANSION_ID=2
export UID_SALT=SomeSalt
export OPT_IN_MODE=false