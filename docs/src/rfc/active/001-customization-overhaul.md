# Feature Name: customization-overhaul

## Summary

Customization in *Fonts of Power* has some fundamental issues centering around character identity and analysis paralysis.
With a much better idea of the core systems of the game, we're ready to take a holistic look at how the pieces fit together.

## Motivation

Quite a few current customization systems exist. Our current analysis of these systems shows several critical problems, particularly around gear:

1. Species
   1. Very limited impact: often feels like a flavor choice and becomes irrelevant at higher levels
   2. No species-specific hooks exist
2. Class
   1. Subclasses are cool, but characters of the same subclass often end up feeling very similar
   2. No class-specific hooks exist
3. Power selection
   1. Very interesting decisions
   2. Only some classes have powers to select
   3. Some analysis paralysis exists
4. Combat talent
   1. Very cool and character defining
   2. Overlap with gear affixes is confusing
   3. No way to get more at higher levels except through gear affixes (weird from a ludonarrative perspective)
5. Skill talent
   1. Really interesting flavor choice with fringe adventure benefits
6. Skill proficiencies
   1. Very interesting for combat, problem solving and flavor
   2. Number of skill proficiencies is small enough to feel like a meaningful customization, but large enough to allow for exploration
   3. Fail-forward rules really help these feel impactful without causing huge number gaps like in Pathfinder
7. Attributes
   1. Very interesting trade-offs for most classes
   2. Easy to understand and allocate
   3. Rolled attributes are a great character seed
8. Ideals
   1. Fantastic flavor hook for both players and NPCs
   2. Fairly simple mechanical integration with Deals and Ideals
9. Gear
   1. Overwhelming complexity
      1. Largely due to interactions and number of options
      2. Promotes intense theory-crafting that make it hard to adapt to realities of gameplay
      3. Managing gear on the tabletop is very challenging; creation is nearly impossible without specialized digital tools
      4. Swapping gear can completely change playstyles
   2. Character identity weakening: abilities are defined by the gear they have, not who they are
      1. Swapping gear makes your character feel very different, reducing willingness to experiment due to flavor concerns
      2. Free selection of affixes really weakens the experience of discovering who your character is
   3. Balancing difficulties
      1. Some affixes are very challenging to balance between different classes
      2. Negative and 0 cost affixes are prone to munchkin-like abuse and can lead to very high complexity
      3. Crafting gear takes a huge amount of downtime with so-so roleplaying impact
      4. Ease of swapping increases ability to hard counter specific encounters, particularly when combined with free selection
      5. Getting new gear is not very impactful due to already-optimized builds
      6. Gear and essence crystal economy is very important due to power budget tied up in gear
   4. Miscellaneous
      1. Half-point affixes are clunky
      2. Problems with gear cascade to monster building
      3. Free selection of affixes limits player interest in testing out alternative affixes in a niche they care about
10. Consumable choice
    1. Complexity, character identity and balancing issues of gear, but weaker
    2. "Signature consumable" pattern is not supported very well
    3. Discovering new consumables isn't exciting due to free selection of affixes

The rolling respec on level-up is a fantastic addition to almost all of these choices, and helps them feel like part of your character without being truly permanent.
As a result, gear feels both more persistent than other character customizations because it costs a scarce resource (downtime activities) to create, but also less persistent, since you can tweak it as many times as you'd like or instantly pick up a new piece of gear.

## Sub-proposal: Shift budget from gear to character customization

### Guide-level explanation

Most of your character's customization (and power) comes from their character choices, rather than the gear they find or make.
By selecting **feats**.

You can select any **feat** from your class and species features when you gain one due to levelling up.
These interact directly with your innate features, while **gear affixes** are simpler complements to your build,
intended to help you adapt to the situation at hand and work for any class and species.

### Reference-level explanation

1. Combat talents are removed, and replaced with feats.
2. Skill talents are renamed to **skill focus**. These are moved to be attached to the specific skill.
3. Gear affix point budget is scaled down.
   1. Equal to proficiency bonus + 2.
   2. Double the cost of all affixes, solving the 0.5 cost affixes as well.
4. Gain one feat at level 1, and one more every ~4 levels (when your attributes increase).
5. When you gain a feat, select from one of the following lists:
   1. A feat associated with your class or subclass.
   2. A feat associated with your species or subspecies.
   3. An affix with a gear point cost of 2 that you know.
6. When you're resurrected as a new species, you must replace all species-specific feats with ones from your new species.

### Drawbacks

1. Another customization system to learn
2. Requires creation of class / species specific content
   1. More work
   2. More space, but see the "effect cards" subproposal below
   3. Increases barrier to homebrew
      1. Mitigated by generic feat access through gear affixes
3. Requires significant balancing and design skill to account for possible interactions between feats
4. Mild analysis paralysis when choosing feats
5. No more "dead-simple" level-ups

### Open questions

1. Do we want a gating mechanism for feats as well?

### Future work

Monster scaling will have to be completely rebalanced to account for this.

Tons of new design space is opened up for classes and species design, but more content will need to be created.

## Sub-proposal: Capstone-like customization

### Guide-level explanation

Powerful magical artifacts are an important fantasy, driving valuable plots that fit well with FoP's dungeon-delving core.
They give players something to strive after, and something to dump their resources into.

Extremely powerful and unique gear effects also open up very interesting gameplay effects.
This is particularly valuable for monster building.

Class capstones serve a valuable role already: clearly defining the class fantasy and giving players something to work towards.
These should stay as is.

### Reference-level explanation

1. Rename prismatic affixes to **signature affixes**.
2. At level 17+, you can use one signature affix on each of arms and armor.
   1. These take up their own slot: do not compete with other affixes.
3. Signature affixes cost lots of essence crystals to craft.
4. Signature affixes must be discovered.
   1. Recipes are destroyed once used.

### Drawbacks

1. Signature affixes are hard to balance.
2. Signature affixes result in very large power spikes.
   1. This power is not evenly spread out across the party.

### Open questions

1. Do we want a feat-analogue for signature affixes?
   1. Sixfold says yes (they're super cool), Alice says no (capstones already exist).
   2. Very unclear where this goes.
   3. Why not just allow class / species specific signature affixes? Do homebrew-only to start.
2. How exactly should signature affixes be discovered?
3. Are we sure about level gating this? Perhaps they should have a different cost.

### Future work

Monsters should have access to signature affixes.

## Sub-proposal: Feature cards

### Guide-level explanation

### Reference-level explanation

### Drawbacks

### Open questions

### Future work

## Sub-proposal: Limited affix pools

### Guide-level explanation

### Reference-level explanation

### Drawbacks

### Open questions

### Future work

## Sub-proposal: Flaws and Drawbacks

### Guide-level explanation

### Reference-level explanation

### Drawbacks

### Open questions

### Future work

## Sub-proposal: Consumable overhaul

### Guide-level explanation

### Reference-level explanation

### Drawbacks

### Open questions

### Future work
