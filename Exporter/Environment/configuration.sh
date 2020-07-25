export CHARACTERS_URL=mysql://root:vagrant@172.17.0.1/characters
export ROCKET_DATABASES="{characters={url=\"${CHARACTERS_URL}\"}, lp_consent={url=\"${LP_CONSENT_URL}\"}}"
export LP_API_TOKEN=a7c8bdf4eaa23de1ab8489190237b7532a15b3779fe9a92914994d2c4528f1af8c434608d1dc6f3b359d009bc268506d20be9855b1100609c852e89745eb1754
export URL_AUTHORIZATION_ENDPOINT=http://localhost:8001/token_validator
export CHARACTER_FETCH_INTERVAL_IN_SEC=300
export EXPANSION_ID=2
export UID_SALT=SomeSalt
export OPT_IN_MODE=false