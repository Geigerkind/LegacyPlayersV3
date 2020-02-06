# Introduction
LegacyPlayers gives you the ability to export and manage your server specific data yourself!
* Keep the data up to date, delete or modify it
* Maintain a history of your player's character's developments
* Provide more detailed data that is otherwise not accessible through player uploads
* Use LPs armory, PvP and PvE log viewer as a service by your server for your player
* Let player manage consent, such that GDPR compliance is ensured

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

TODO: Picture of the consent manager

* The user can specify the characters that the player wants to export
* If a character is the guild master of some guild, the player can specify whether the guild's raid logs are
exported
* The player can choose to delete data of a character
* The player can choose to delete guild raid logs that the player's characters are the guild master of

Given these information, data that is exported, through the armory or world server exporter, is filtered
and then optionally send to LegacyPlayers, where the player can view its data immediately. 

# Which endpoint needs to be implemented?
TODO

# How can the world server plugin be installed?
TODO

# How do I install and configure the docker container?
TODO