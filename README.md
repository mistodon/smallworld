Small World - LD 38
===

I plan to keep notes on the game as I go along, written below.

[2017-04-22 02:54]:
Created repo. I have no idea what I'm going to do yet, but I might as well lay some groundwork.

[2017-04-22 03:03]:
While I get some boilerplate done, I'm wondering what mechanics fit with the theme of a small world.

I think I'm more likely to go with the literal small world than the figurative, since I'm not sure I can make an interesting game about meeting people by coincidence in 48 hours.

I'm thinking maybe some kind of puzzle game based around managing limited space in an enclosed location.

[2017-04-22 03:13]:
Got a window up, clearing, and closing properly.

[2017-04-22 03:27]:
Got a "game state" which fades in some blue and then exits. Probably going to go to bed soon.

Before I do, here's my basic working idea:

Each stage is a small grid-based map. The map contains some number of people in predetermined locations. The map also contains things like rocks, trees, water, etc. Obstacles mostly. You have to last a certain number of days while keeping at least one person alive. Each day, people move one tile. Each day, trees produce fruit. Each day that there is a deficit of food, somebody dies. Each day that a person has no tile that they can move to, they die.

The player can control the movement of the people, the amount of space remaining, and the amount of food present by placing and removing objects (like trees, rocks, etc.)

Not sure how much I like this idea, but we'll see how I feel in a bit.

[2017-04-22 03:40]:
Note to self, don't use absurd formatting in markdown files.

[2017-04-22 14:27]:
Alright, slept in today, but I think I have a stronger idea for what I want to do now. I'll describe it as I go along.

[2017-04-22 14:59]:
Got shaders loading - but not doing anything with them yet. The `find_folder` crate, and the `cargo check` command are insanely useful.

[2017-04-22 15:09]:
Build a quad mesh, drawing it on screeeen.

It's incredibly likely that this whole game is going to use one mesh, one shader, and one texture.

[2017-04-22 15:37]:
Load texture, render it onto quad, blah blah.

[2017-04-22 15:48]:
Correct projection and display textures at pixel scale (unless I fucked something up, we'll see). Treating 1.0 unit of world space as a 16px tile. Gonna take a break now.

[2017-04-22 16:02]:
Next up, some actual game state. I'm going to use [specs](https://github.com/slide-rs/specs) for this. I've played around implementing entity systems before, but I really don't feel like I've got enough time to do it right. Plus specs is almost certainly better thought out than whatever I could throw together.

[2017-04-22 16:23]:
Got multiple entities on screen now. Not really using a proper "system" for it, since it needs to be on the main thread, but whatever, it's a similar idea.

[2017-04-22 17:12]:
Added motion system from a position to a destination. Also spent probably too long on vector operators - use a stock library next time.

[2017-04-22 17:43]:
Very basic player movement. You can slide around the screen with the arrow keys.
