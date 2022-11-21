# Type Mecahnics
In Rustymon, there are several different types of creatures; each creature can have one or two types that inform the effectiveness of matching move types when used in battle.
Below is a list of possible types and a matchup-chart to display effectiveness of everything.

*Zen Type*:

"Dervied from the peaceful essense of nature," these creatures employ precise moves utilizing their natural surrounding for effective combat.
Zen movesets are most effective in the context of defensive strategy, weakening the abilities of their opponents before delivering quick, effective blows.

Exclusively Zen-typed creatures are uncommon; instead, Zen will often be paired with other types. 

## Most of the **Elemental** types are offense-focused, and reflect on different foundational aspects of the natural universe.

*Fire Type*:

Fire types are considered to be one of the oldest elemental types, birthed from the supernoval death of stars accross the universe.
Fire movesets are fast and attack focused, able to deliver direct attacks multiple times over to weaken opponents quickly.  

*Water Type*:

Water types are the antithesis to fire types, and bring a more balanced approach to combat. Creatures that are water typed will have the advantage against singly-typed fire creatures, but are generally slower, and err on the defensive more often than fire-types would. They have a varied moveset with lots of opportunity for training your water-type creature to match your style of cambat, be it defensive or offensive. Overall, water types are very well-rounded of the four elemental types. 

*Earth Type*:

Earth types are super-defensive, having the ability to withstand/dilute attacks from a wide-variety of enemy types. They're generally much slower creatures than other types, and when they deliver attacks they will be powerful but hard to land. 

*Wind Type*:

Wind types are extremely weak defensively, but are one of the most technically adept attackers (and fastest movers). Their moveset includes a huge variety of different attack strategies, both physical and specialized. Depending on the environment they are in, they have a chance to be immobilized or greatly weakened or, in rare circumstances, their environment could increase thier speed.

## Other creature types are derived from a variety of aspects that make up our universe.

*Astral/Cosmic Type*:

Astral typed creatures are ancient, and their origins are not well-known. What we do know is that they exist primarily in space, but occasionally settle onto certain planets for extended eras. These creatures are very fast, but generally weak, and their moveset is limited to exclusively special-types.  

*Time Type*:

There are only two known creatures in our universe with this type, and both are exceedingly rare. With the ability to manipulate time through certain moves, they make great support members on a team. On their own, though, they will not fare well against strong, high-HP opponents. 

*Dark and Light Types*:

Dark and Light Types' movesets are completely ineffective against one another, but both have specific characteristics that give them advantages against other types. For example, dark and light types are weak against Earth types, but dark is effective against wind and light is effective against water types.

# Statistic Mecahnics

- Health Points (HP): Health-depletion is the core component to pokemon battles; if a creature's health drops to zero, they faint and must be revived. Health point totals can range from ~15 to greater than 100, and tends to scale with a creature's experience level.

- Attack: The attack statistic determines foundationally how much damage a certain move does.

- Defense: The defense statistic determines foundationally how much damage other moves do to itself.

- Speed: This statistic determines the first-actor in a battle, but in creature battles this statistic might be shadowed by special moves, abilities, etc. Speed also determines the creature's ability to evade attacks of others.

- Accuracy: This statistic determines the accuracy of a certain move, ie, the chance that it will land a hit on the opposing creature. This statistic is not specific to a single creature, and is instead reset to 100% at the beginning of each battle. Moves themselves will also have an accuracy statistic that alters the chance of a move landing.

# Battle Mechanics

Battles in Rustymon can occur between a minimum of two creatures, or can be paired battles (2 v 2). Battles are turn-based and the creature who attacks first is based on a variety of factors (in order of precidence):
    
1. A given creature's calculated speed statistics are the first indicator of who attacks first, unless players are switching out to different creatures which will always happen first. *See above Statistic Mechanics section for information about how the speed is calculated.*

2. Eventually, we will flesh-out specfic abilities which can give creatures additional priority in battle. That information will go here.

3. If the speed statistics are the same for all creatures, the game will break the tie at random.

Critical hits occur based on the critical hit stage ratio during a battle. This ratio is calculated based on a stage variable that might increase based on move's critical hit ratio (CHR), the ability of certain creatures, the affect of certain settings, and the ability of certain item, and create friendliness.

The base critical hit ratio is 1:16.

Damage taken is calculated from on a base formula (framed after the calculation in later generations of Pokemon):

    D = floor(floor(floor((2 * Level / 5) + 2) * Power * Attack / Defense) / 50) + 2

Level is the level of the creature. Attack is the attack multiplier statistic. Power is the move's power statistic. Defense is the defense statistic of the opposing creature. 

# Interaction Mechanics

- Capturing: The creatures are captured and released via totems, imbued with power from the spirits. Totems are not purchasable, but they can be earned from spirits and sometimes found lying around, forgotten. Totems can be upgraded, either by spirits themselves or those with the blessing of a spirit.
- Encountering: Specific tiles will hide the creatures. Upon walking in them, there is a chance that one will encounter a creature. The kind of creature encountered will depend on the area of the world that the player is in. The player can also call upon the spririts to endow him with an attracting energy that will make creature encounters more common, and possibly occur in tiles that do not hide the creatures.