local RPLL = RPLL
RPLL.VERSION = 1
RPLL.PlayerInformation = {}
RPLL.PlayerRotation = {}
RPLL.RotationIndex = 1
RPLL.RotationLength = 0

RPLL:RegisterEvent("PLAYER_TARGET_CHANGED")
RPLL:RegisterEvent("RAID_ROSTER_UPDATE")
RPLL:RegisterEvent("PARTY_MEMBERS_CHANGED")

RPLL:RegisterEvent("ZONE_CHANGED_NEW_AREA")
RPLL:RegisterEvent("UPDATE_INSTANCE_INFO")

RPLL:RegisterEvent("UPDATE_MOUSEOVER_UNIT")
RPLL:RegisterEvent("PLAYER_ENTERING_WORLD")
RPLL:RegisterEvent("VARIABLES_LOADED")

RPLL:RegisterEvent("UNIT_PET")
RPLL:RegisterEvent("PLAYER_PET_CHANGED")
RPLL:RegisterEvent("PET_STABLE_CLOSED")

RPLL:RegisterEvent("UI_ERROR_MESSAGE")
RPLL:RegisterEvent("UI_INFO_MESSAGE")

local tinsert = table.insert
local strformat = string.format
local GetTime = GetTime
local UnitName = UnitName
local strgfind = string.gfind
local strsub = string.sub
local GetNumSavedInstances = GetNumSavedInstances
local GetSavedInstanceInfo = GetSavedInstanceInfo
local IsInInstance = IsInInstance
local pairs = pairs
local GetNumPartyMembers = GetNumPartyMembers
local GetNumRaidMembers = GetNumRaidMembers
local UnitHealth = UnitHealth
local UnitIsPlayer = UnitIsPlayer
local UnitLevel = UnitLevel
local UnitSex = UnitSex
local strlower = strlower
local GetInspectHonorData = GetInspectHonorData
local GetGuildInfo = GetGuildInfo
local GetInspectPVPRankProgress = GetInspectPVPRankProgress
local GetInventoryItemLink = GetInventoryItemLink
local GetPVPRankInfo = GetPVPRankInfo
local UnitPVPRank = UnitPVPRank
local strfind = string.find
local Unknown = UNKNOWN
local LoggingCombat = LoggingCombat

RPLL.ZONE_CHANGED_NEW_AREA = function()
    LoggingCombat(IsInInstance("player"))
    this:grab_unit_information("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
end

RPLL.UPDATE_INSTANCE_INFO = function()
    LoggingCombat(IsInInstance("player"))
    this:grab_unit_information("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
end

RPLL.PLAYER_ENTERING_WORLD = function()
    this:grab_unit_information("player")
end

RPLL.VARIABLES_LOADED = function()
    this:grab_unit_information("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
end

RPLL.PLAYER_TARGET_CHANGED = function()
    this:grab_unit_information("target")
end

RPLL.UPDATE_MOUSEOVER_UNIT = function()
    this:grab_unit_information("mouseover")
end

RPLL.RAID_ROSTER_UPDATE = function()
    for i=1, GetNumRaidMembers() do
        if UnitName("raid"..i) then
            this:grab_unit_information("raid"..i)
        end
    end
end


RPLL.PARTY_MEMBERS_CHANGED = function()
    for i=1, GetNumPartyMembers() do
        if UnitName("party"..i) then
            this:grab_unit_information("party"..i)
        end
    end
end

RPLL.UNIT_PET = function(unit)
    if unit then
        this:grab_unit_information(unit)
    end
end

RPLL.PLAYER_PET_CHANGED = function()
    this:grab_unit_information("player")
end

RPLL.PET_STABLE_CLOSED = function()
    this:grab_unit_information("player")
end

RPLL.UI_INFO_MESSAGE = function()
    this:rotate_combat_log_global_string()
end

RPLL.UI_ERROR_MESSAGE = function()
    this:rotate_combat_log_global_string()
end

function RPLL:grab_unit_information(unit)
    local unit_name = UnitName(unit)
    if UnitIsPlayer(unit) and unit_name ~= nil and unit_name ~= Unknown then
        if this.PlayerInformation[unit_name] == nil then
            this.PlayerInformation[unit_name] = {}
            tinsert(this.PlayerRotation, unit_name)
            this.RotationLength = this.RotationLength + 1
        end
        local info = this.PlayerInformation[unit_name]
        info["name"] = unit_name

        -- Guild info
        local guildName, guildRankName, guildRankIndex = GetGuildInfo(unit)
        info["guild_name"] = guildName
        info["guild_rank_name"] = guildRankName
        info["guild_rank_index"] = guildRankIndex

        -- Pet name
        if strfind(unit, "pet") == nil then
            local pet_name = nil
            if unit == "player" then
                pet_name = UnitName("pet")
            elseif strfind(unit, "raid") then
                pet_name = UnitName("raidpet"..strsub(unit, 5))
            elseif strfind(unit, "party") then
                pet_name = UnitName("partypet"..strsub(unit, 6))
            end

            if pet_name ~= nil and pet_name ~= Unknown then
                info["pet"] = pet_name
            end
        end

        -- Hero Class, race, sex
        info["hero_class"] = UnitClass(unit)
        info["race"] = UnitRace(unit)
        info["sex"] = UnitSex(unit)

        -- Gear
        info["gear"] = {}
        for i=1, 19 do
            local inv_link = GetInventoryItemLink(unit, i)
            if inv_link == nil then
                info["gear"][i] = nil
            else
                local found, _, itemString = strfind(inv_link, "Hitem:(.+)\124h%[")
                if found == nil then
                    info["gear"][i] = nil
                else
                    info["gear"][i] = itemString
                end
            end
        end
    end
end

function RPLL:rotate_combat_log_global_string()
    if this.RotationLength ~= 0 then
        local character = this.PlayerInformation[this.PlayerRotation[this.RotationIndex]]
        local result = "COMBATANT_INFO: "
        local gear_str = prep_value(character["gear"][1])
        for i=2, 19 do
            gear_str = gear_str..","..prep_value(character["gear"][i])
        end
        result = result..prep_value(character["name"])..","..prep_value(character["hero_class"])..","..prep_value(character["race"])..","..prep_value(character["sex"])..","..prep_value(character["pet"])..","..prep_value(character["guild_name"])..","..prep_value(character["guild_rank_name"])..","..prep_value(character["guild_rank_index"])..","..gear_str
        SPELLFAILCASTSELF = result
        SPELLFAILPERFORMSELF = result
        if this.RotationIndex + 1 > this.RotationLength then
            this.RotationIndex = 1
        else
            this.RotationIndex = this.RotationIndex + 1
        end
    end
end

function prep_value(val)
    if val == nil then
        return "nil"
    end
    return val
end