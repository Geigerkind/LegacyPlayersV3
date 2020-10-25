local RPLL = RPLL
RPLL.VERSION = 3
RPLL.PlayerInformation = {}

local strsplit = strsplit
local strjoin = strjoin
local tinsert = table.insert
local UnitName = UnitName
local UnitSex = UnitSex
local UnitRace = UnitRace
local UnitClass = UnitClass
local GetGuildInfo = GetGuildInfo
local GetInventoryItemLink = GetInventoryItemLink
local UnitGUID = UnitGUID
local IsInInstance = IsInInstance
local UnitIsPlayer = UnitIsPlayer
local strfind = string.find
local strsub = string.sub
local GetNumTalents = GetNumTalents
local GetTalentInfo = GetTalentInfo
local GetNumRaidMembers = GetNumRaidMembers
local GetNumPartyMembers = GetNumPartyMembers
local strlen = strlen
local GetArenaTeam = GetArenaTeam
local GetInspectArenaTeamData = GetInspectArenaTeamData
local date = date
local GetRealmName = GetRealmName
local GetNumSavedInstances = GetNumSavedInstances
local GetSavedInstanceInfo = GetSavedInstanceInfo
local unpack = unpack
local strlower = strlower
local GetRealZoneText = GetRealZoneText

RPLL:RegisterEvent("UPDATE_MOUSEOVER_UNIT")
RPLL.UPDATE_MOUSEOVER_UNIT = function()
    this:CollectUnit("mouseover")
end
RPLL:RegisterEvent("PLAYER_TARGET_CHANGED")
RPLL.PLAYER_TARGET_CHANGED = function()
    this:CollectUnit("target")
end
RPLL:RegisterEvent("RAID_ROSTER_UPDATE")
RPLL.RAID_ROSTER_UPDATE = function()
    for i = 1, GetNumRaidMembers() do
        if UnitName("raid" .. i) then
            this:CollectUnit("raid" .. i)
        end
    end
end
RPLL:RegisterEvent("PARTY_MEMBERS_CHANGED")
RPLL.PARTY_MEMBERS_CHANGED = function()
    for i = 1, GetNumPartyMembers() do
        if UnitName("party" .. i) then
            this:CollectUnit("party" .. i)
        end
    end
end

RPLL:RegisterEvent("ZONE_CHANGED_NEW_AREA")
RPLL.ZONE_CHANGED_NEW_AREA = function()
    LoggingCombat(IsInInstance("player"))
    this:PushCurrentInstanceInfo()
end
RPLL:RegisterEvent("UPDATE_INSTANCE_INFO")
RPLL.UPDATE_INSTANCE_INFO = function()
    LoggingCombat(IsInInstance("player"))
    this:PushCurrentInstanceInfo()
end

RPLL:RegisterEvent("UNIT_PET")
RPLL.UNIT_PET = function(unit)
    this:CollectUnit(unit)
end
RPLL:RegisterEvent("UNIT_ENTERED_VEHICLE")
RPLL.UNIT_ENTERED_VEHICLE = function(unit)
    this:CollectUnit(unit)
end

RPLL:RegisterEvent("PLAYER_PET_CHANGED")
RPLL.PLAYER_PET_CHANGED = function()
    this:CollectUnit("player")
end
RPLL:RegisterEvent("PET_STABLE_CLOSED")
RPLL.PET_STABLE_CLOSED = function()
    this:CollectUnit("player")
end

RPLL:RegisterEvent("CHAT_MSG_LOOT")
RPLL.CHAT_MSG_LOOT = function(msg)
    if not IsInInstance() then
        return
    end
    this:PushExtraMessage(strjoin("&", "LOOT",msg))
end

RPLL:RegisterEvent("PLAYER_ENTERING_WORLD")
RPLL.PLAYER_ENTERING_WORLD = function()
    if RPLL_PlayerInformation == nil then
        RPLL_PlayerInformation = {}
    end

    if RPLL_ExtraMessages == nil then
        RPLL_ExtraMessages = {}
    end

    if GetRealmName() ~= RPLL_CurrentServer then
        RPLL_PlayerInformation = {}
        RPLL.PlayerInformation = {}
        RPLL_ExtraMessages = {}
        RPLL_CurrentServer = GetRealmName()
    end

    local player_name = UnitName("player")

    LOOT_ITEM_PUSHED_SELF = player_name.." receives item: %sx1."
    LOOT_ITEM_PUSHED_SELF_MULTIPLE = player_name.." receives item: %sx%d."
    LOOT_ITEM_SELF = player_name.." receives loot: %sx1."
    LOOT_ITEM = "%s receives loot: %sx1."
    LOOT_ITEM_SELF_MULTIPLE = player_name.." receives loot: %sx%d."

    SLASH_rpll1 = "/rpll"
    SLASH_rpll2 = "/RPLL"
    SlashCmdList["rpll"] = function(msg)
        if msg == "nuke" then
            RPLL_PlayerInformation = {}
            RPLL.PlayerInformation = {}
            RPLL_ExtraMessages = {}
            RPLL_CurrentServer = GetRealmName()
            print("Log nuked");
        else
            print("LegacyPlayers: To nuke a log type: /rpll nuke!");
        end
    end

    print("LegacyPlayers collector v"..RPLL.VERSION.." has been loaded. Type /rpll for help.")

    this:LoadPlayerInformation()
    this:CollectUnit("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
end

RPLL:RegisterEvent("PLAYER_LOGOUT")
RPLL.PLAYER_LOGOUT = function()
    this:SavePlayerInformation()
end

function RPLL:LoadPlayerInformation()
    if RPLL_PlayerInformation == nil then
        RPLL_PlayerInformation = {}
    end

    for key, val in pairs(RPLL_PlayerInformation) do
        if key ~= nil and val ~= nil then
            local unit_guid, unit_name, unit_race, hero_class, gender, guild_name, guild_rank_name, guild_rank_index, pet_guids_str, gear_str, talents, arena_team2, arena_team3, arena_team5 = strsplit("&", val)
            local gear = { strsplit("}", gear_str) }
            local pet_guids = { strsplit("}", pet_guids_str) }
            local arena_teams = {}
            if arena_team2 ~= nil then
                arena_teams[2] = arena_team2
            end
            if arena_team3 ~= nil then
                arena_teams[3] = arena_team3
            end
            if arena_team5 ~= nil then
                arena_teams[5] = arena_team5
            end

            this:UpdatePlayer(unit_guid, unit_name, unit_race, hero_class, gender, guild_name, guild_rank_name, guild_rank_index, pet_guids, gear, talents, arena_teams)
        end
    end
end

function RPLL:SavePlayerInformation()
    RPLL_PlayerInformation = {}
    for key, val in pairs(this.PlayerInformation) do
        if key ~= nil and val ~= nil then
            local pets = prep_value(val["pet_guids"][1])
            for i=2, table_len(val["pet_guids"]) do
                pets = pets.."}"..prep_value(val["pet_guids"][i])
            end

            local gear = prep_value(val["gear"][1])
            for i=2, 19 do
                gear = gear.."}"..prep_value(val["gear"][i])
            end

            RPLL_PlayerInformation[key] = strjoin("&", prep_value(val["unit_guid"]), prep_value(val["unit_name"]),
                    prep_value(val["race"]), prep_value(val["hero_class"]), prep_value(val["gender"]), prep_value(val["guild_name"]),
                    prep_value(val["guild_rank_name"]), prep_value(val["guild_rank_index"]), pets, gear, prep_value(val["talents"]),
                    prep_value(val["arena_teams"][2]), prep_value(val["arena_teams"][3]), prep_value(val["arena_teams"][5]))
        end
    end
end

function RPLL:UpdatePlayer(unit_guid, unit_name, race, hero_class, gender, guild_name, guild_rank_name, guild_rank_index, pet_guids, gear, talents, arena_teams)
    if unit_guid == nil or unit_name == nil then
        return
    end

    if this.PlayerInformation[unit_guid] == nil then
        this.PlayerInformation[unit_guid] = {}
    end

    local info = this.PlayerInformation[unit_guid]
    info["unit_guid"] = unit_guid
    info["unit_name"] = unit_name

    if race ~= nil then
        info["race"] = race
    end

    if hero_class ~= nil then
        info["hero_class"] = hero_class
    end

    if gender ~= nil then
        info["gender"] = gender
    end

    if guild_name ~= nil and guild_rank_name ~= nil then
        info["guild_name"] = guild_name
        info["guild_rank_name"] = guild_rank_name
        info["guild_rank_index"] = guild_rank_index
    end

    if pet_guids ~= nil then
        if info["pet_guids"] == nil then
            info["pet_guids"] = {}
        end

        for _, val in pairs(pet_guids) do
            if not table_contains(info["pet_guids"], val) then
                tinsert(info["pet_guids"], val)
            end
        end
    end

    if info["gear"] == nil then
        info["gear"] = {}
    end
    if gear ~= nil then
        local any_item = false
        for i = 1, 19 do
            if gear[i] ~= nil then
                any_item = true
                break
            end
        end

        if any_item then
            info["gear"] = gear
        end
    end

    if talents ~= nil then
        info["talents"] = talents
    end

    info["arena_teams"] = {}
    for team_size, team_name in pairs(arena_teams) do
        info["arena_teams"][team_size] = team_name
    end
end

function RPLL:CollectUnit(unit)
    if not UnitIsPlayer(unit) then
        return
    end
    if UnitGUID(unit) ~= nil and UnitGUID(unit) == UnitGUID("player") then
        unit = "player"
    end
    local unit_guid = UnitGUID(unit)
    local unit_name = UnitName(unit)
    local unit_race = UnitRace(unit)
    local unit_hero_class = UnitClass(unit)
    local unit_gender = UnitSex(unit)
    local guild_name, guild_rank_name, guild_rank_index = GetGuildInfo(unit)
    local pet_guid = nil;
    if unit == "player" then
        pet_guid = UnitGUID("pet")
    elseif strfind(unit, "raid") then
        pet_guid = UnitGUID("raidpet" .. strsub(unit, 5))
    elseif strfind(unit, "party") then
        pet_guid = UnitGUID("partypet" .. strsub(unit, 6))
    end

    local gear = {}
    for i = 1, 19 do
        local inv_link = GetInventoryItemLink(unit, i)
        if inv_link == nil then
            gear[i] = nil
        else
            local _, itemId, enchantId, jewelId1, jewelId2, jewelId3,
            jewelId4, suffixId, uniqueId = strsplit(":", inv_link)
            if itemId == nil then
                gear[i] = nil
            else
                gear[i] = strjoin(":", itemId, enchantId, jewelId1, jewelId2, jewelId3, jewelId4, suffixId, uniqueId)
            end
        end
    end

    local talents = {"", "", ""};
    for t = 1, 3 do
        local numTalents = GetNumTalents(t, unit_guid ~= UnitGUID("player"));
        -- Last one is missing?
        for i = 1, numTalents do
            local _, _, _, _, currRank = GetTalentInfo(t, i, unit_guid ~= UnitGUID("player"));
            talents[t] = talents[t] .. currRank
        end
    end
    talents = strjoin("}", talents[1], talents[2], talents[3])
    if strlen(talents) <= 10 then
        talents = nil
    end

    local arena_teams = {}
    for i=1, 3 do
        local team_name, team_size
        if unit == "player" then
            team_name, team_size = GetArenaTeam(i);
        elseif unit == "target" then
            team_name, team_size = GetInspectArenaTeamData(i);
        end

        if team_name ~= nil and team_size ~= nil then
            arena_teams[team_size] = team_name
        end
    end

    this:UpdatePlayer(unit_guid, unit_name, unit_race, unit_hero_class, unit_gender, guild_name, guild_rank_name, guild_rank_index, { [1] = pet_guid }, gear, talents, arena_teams)
end

function RPLL:PushExtraMessage(msg)
    if RPLL_ExtraMessages == nil then
        RPLL_ExtraMessages = {}
    end
    tinsert(RPLL_ExtraMessages, date("%d.%m.%y %H:%M:%S").."&"..msg)
end

function RPLL:PushCurrentInstanceInfo()
    if not IsInInstance() then
        return
    end

    local zone = strlower(GetRealZoneText())
    for i=1, GetNumSavedInstances() do
        local instance_name, instance_id = GetSavedInstanceInfo(i)
        if zone == strlower(instance_name) then
            local participants = {}
            tinsert(participants, UnitGUID("player"))
            for j = 1, GetNumRaidMembers() do
                if UnitName("raid" .. j) then
                    tinsert(participants, UnitGUID("raid"..j))
                end
            end
            for j = 1, GetNumPartyMembers() do
                if UnitName("party" .. j) then
                    tinsert(participants, UnitGUID("party"..j))
                end
            end
            this:PushExtraMessage(strjoin("&", "ZONE_INFO", instance_name, instance_id, unpack(participants)))
            break
        end
    end
end

function prep_value(val)
    if val == nil then
        return "nil"
    end
    return val
end

function table_contains(table, val)
    for _, v in pairs(table) do
        if v == val then
            return true
        end
    end
    return false
end

function table_len(table)
    local len = 0
    for _, _ in pairs(table) do
        len = len + 1
    end
    return len
end