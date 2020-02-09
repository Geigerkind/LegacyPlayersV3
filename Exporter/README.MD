# Introduction
LegacyPlayers gives you the ability to export and manage your server specific data yourself!
* Keep the data up to date, delete or modify it
* Maintain a history of your player's character's developments
* Provide more detailed data that is otherwise not accessible through player uploads
* Use LPs armory, PvP and PvE log viewer as a service by your server for your player
* Let player manage consent, such that GDPR compliance is ensured

# Disclaimer
This is merely a suggestion  of an implementation for the export mechanism with privacy in mind.
Feel free to fork this exporter and modify it such that it suits your needs. You can also implement
of fully custom system using the API specified on the website's swagger UI.

# How does it work?
LegacyPlayers requires you to implement two endpoints and mount a docker container in your server environment.
* A plugin for your world server that exports raid and pvp data to the container
* An endpoint for the consent manager that maps an account token to the account id, which is used to link
character ids to an account

The docker container requires access to character DB, such that it can export the data of players that gave
consent. The picture below outlines its functionality.

![LP-Export-Functionality](LP-Export-Functionality.png)

You need to implement in your website's account panel a link to the docker container's consent manager
(how is specified below). The consent manager takes care of the rest:  

![ConsentManager](ConsentManager.png)

* The player can specify the characters that the player wants to export
* If a character is the guild master of some guild, the player can specify whether the guild's raid logs are
exported
* The player can choose to delete data of a character
* The player can choose to delete guild raid logs that the player's characters are the guild master of

Given these information, data that is exported, through the armory or world server exporter, is filtered
and then optionally send to LegacyPlayers, where the player can view its data immediately. 

# Installation
Before referring to the consent manager, set in the local storage for the key `API_TOKEN` the
serialized JSON object `{ token: string; account_id: number; }`.

Specify the environment variable `URL_AUTHORIZATION_ENDPOINT` and `LP_API_TOKEN` in the docker-compose file.

The `LP_API_TOKEN` refers to the API token generated on LegacyPlayers and the endpoint 
verifies the key,value pair specified in `API_TOKEN`. The backend server will attempt to
post to `URL_AUTHORIZATION_ENDPOINT`. The endpoint should return either 
`true` or `false`. Please check out the sample endpoint in the export repository for more 
information.

The container requires `read-only access` to your WoW-server's `character database`. Please 
create a database user for this purpose and specify the DNS in the environment variable 
`CHARACTER_MYSQL_DNS` in the docker-compose file.

Further you need to specify the following environment variables in the docker-compose file:
* `EXPANSION_ID` - Vanilla => 1; TBC => 2; WOTLK => 3. If your server harbors a custom implementation 
of WoW or your expansion is not among these, please contact me via Discord.
* `UID_SALT` - Your character und guild guids are not send directly to LP. They are hashed 
using the provided salt, as it is only required for you to identify these characters. Please 
do not loose this salt, because it is not recoverable, nor can any character be re-guided.
* `CHARACTER_FETCH_INTERVAL_IN_SEC` - Per default, every 60 your character database is fetched 
for characters that went offline since the last fetch. You can specify this interval here.

Once everything is done, you can start the container using `docker-compose up`. If your 
server is often subject to a restart, you should also create a systemd service that starts 
the consent manager as well. 

Initially the armory exporter will fully fetch all offline characters and queue them to be 
send to LegacyPlayers, so there may be a small spike. 

Also every night at 5 AM (CET), the service will update itself and in that process prolong 
your API token on LP for a few days such that it will not expire.