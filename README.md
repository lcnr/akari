# Akari

A showcase game for the `crow` engine, currently work in progress.

## Design

A high intensity 2D hack and slay platformer with a large focus on dodging attacks.

The player has only one health point and quickly respawns. The spawn points are always fairly close and
one should always be able to get back to a previous location with close to no downtime. All enemy attacks
have to be both survivable and have a visible start up.

- downtime: time spend where no decisions are required, for example walking, waiting for cycles/events and long dialogs/cutscenes
- faily close: less than 30 seconds to return to the location of the last death

## Story

There is an evil sorcerer who consumes the soul of all living beings. After failing to defeat the sorcerer herself, the goddess grants
the player character her remaining power. This allows the player to respawn at her shrines and be temporarily invincible during dashes.
If the player did not consume enough of the goddesses power, he dies during the last transformation of the sorcerer, due to it's effect on
the soul.

## Design goals

The game is as straightforward and obvious as possible, the player is a cute and nimble hero (idc about gender) who challenges the dark and evil
sorcerer, who is evil. Nearly all enemies are clearly evil and are by design slower, stronger and bigger than the player (and they always defeat the
player in one hit). If everything important can be understood in one image, I did a good job.

## Expected Playtime

- first playthrough: 2 hours
- for seasoned players: 20 minutes or less

## Gameplay

The player can only move, attack with a quick melee combo and perform a dodge roll.
He always has exactly one health and dies instantly.
Walking past shrines automatically updates the spawn point. In case he
wants to take additional risks he can break them, which destroys them and causes
him to absorb the contained magic power of the goddess which slightly increases his attack.
If the player destroyed a given amount of shrines he is able to survive the beginning of
the last stage of the last boss, which results in a true ending.

## Maybe

The player gets a protective shild/spirit after praying/respawning at a shrine, blocking one hit