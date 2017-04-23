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

[2017-04-22 18:08]:
Sprites now have a texture region so that they can sample the atlas at different points. Rather than make each entity have its own mesh with its own UVs, I'm just putting a UV offset/scale in the vertex shader.

[2017-04-22 18:20]:
Added some simple collision with blocks.

[2017-04-22 18:30]:
Texture artefacts were annoying me so I disabled mipmaps (which didn't help) and now I'm rounding the position of each sprite to the nearest "pixel" (meaning nearest multiple of {scale} pixels). This seems to have worked. Although now I'm really noticing the jerkiness whenever you reach a tile and stop for one frame. Might fix it if I have time, or maybe I'll get around to animating the sprites and that'll cover it up a bit?

[2017-04-22 19:10]:
Took a break - just started working again. Going to add a lose condition now - when you collide with the other guy.

[2017-04-22 19:32]:
OK so now when you touch the door, or the guy, the level restarts.

[2017-04-22 20:07]:
Took a break, then made things turn-based, so the stalker should move in lockstep with the player.

So now is probably a good time to explain the eventual point of the game. Simple enough: get to the door without coming in contact with the stalker. The stalker will follow your exact movements where possible, which you can use to make a strategy about where you go next.

That's the bare bones of it. If by some miracle I have time to do more, I will.

[2017-04-22 20:31]:
Alright, the shady dude follows you around now, but unfortunately it's impossible to collide with him since you're moving in perfect lockstep (you always change tiles at the same time and usually don't have the exact same destination).

To fix this, I'm probably going to put a slight delay on the stalker's movement - like half a step.

[2017-04-22 21:38]:
Delay seemed to work, though it was messier than I'd hoped. Think it's about time to write a level parser!

[2017-04-22 22:33]:
We can now parse a whole bunch of levels from a yaml file! Hooray! However, this kind of breaks the stalker, who needs initialized with a path to the player. It was previously hardcoded, so I'll need to fill it in automatically. Hopefully this will disappear when a bit of pathfinding gets added. (When?)

[2017-04-22 22:56]:
Stalker path is now fixed, plus the level looks a little prettier now.

[2017-04-22 23:09]:
Sorting sprites by layer now so I can put a door in front of a wall.

[2017-04-23 01:48]:
Added push blocks.

[2017-04-23 03:46]:
Oh geez. So having the movement code for the player and stalker separate turned into a whole horrifying deal where I now have three different places checking to see if something is in the way of a push block.

On the bright side, it seems to work, and I added a couple of levels.

[2017-04-23 04:07]:
To do tomorrow, roughly in priority order:

1.  Fix moving-into-box bug
2.  Splash screen
3.  Ending screen
4.  Music
5.  Sound effects
6.  Buttons/gates
7.  More levels
8.  Animation
9.  Prettier sprites
10. Even more levels
