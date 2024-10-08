# Concept

Out-of-the-box, Cyberpunk 2077's *vanilla*[^1] audio engine is built on top of [Audiokinetic's WWise](https://www.audiokinetic.com/en/wwise/overview/) which is a professional-grade audio software and tools suite.

Audioware *does not make any use of it* and it has *almost* **no** control over it.

Here's how Cyberpunk 2077 is designed, in an overly simplified way.

![Cyberpunk 2077 audio system](./assets/without-audioware.png)

~~~admonish question title="Why not directly hook WWise?"
When I initially started working on Audioware I also was tempted to hook everything from Audiokinetic to allow adding custom audio to the game. At first.
~~~

But the reality is that, when you choose this path on one side you get native[^2] integration which is great, but on the other you then need to do <span style="color: hotpink">everything</span> as both [WWise](https://www.audiokinetic.com/en/library/edge/?source=SDK&id=index.html) and the game does. Not even mentioning that you actually will have to learn how CDPR works <span style="color: hotpink">with</span> WWise, which is [not always standard](https://github.com/vgmstream/vgmstream/issues/778).

Professional *all-in-one* softwares like Audiokinetic can be dauting to use when unfamiliar and quickly become an <span style="color: hotpink">entry-skills barrier[^3]</span> for newcomers who would simply like to add sounds, play around with them, have fun and come up with an interesting mod.

~~~admonish question title="What about REDmod?"
I really enjoyed [REDmod](https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators-theory/modding-tools/redmod/audio-modding) in the beginning but always felt frustrated after a while to not be able to alter sounds dynamically.
~~~

The [audio parameters](https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators-theory/modding-tools/redmod/audio-modding#parameters) it exposes are very cool, but as far as I remember you can't seamlessly switch from one to another, and if you want multiple parameters per sound you basically have to duplicate them as many times.

Last but not least, I regularly got players complaining about it becoming overly slow when (too?) many mods use it, and it does not seem particularly appreciated among player base.

~~~admonish question title="Then, how?"
What I always wanted *right from start* is a tool that can get me going <span style="color: hotpink">in under 15min</span>.

I wanted something to be able to play <span style="color: hotpink">easily defined</span> sounds with <span style="color: hotpink">parameters</span> and <span style="color: hotpink">audio effects</span>.

Something <span style="color: #f3d772">Simple</span>. <span style="color: #f3d772">Easy</span>. yet <span style="color: #f3d772">Customizable</span> and <span style="color: #f3d772">Fast</span>.
~~~

And this how Audioware was initially born as a simple proof-of-concept in [4ddicted](https://github.com/cyb3rpsych0s1s/4ddicted), another mod of mine. Until other modders started to notice that it worked pretty well and asked me to turn into a fully integrated native plugin.

Audioware actually uses a second <span style="color: hotpink">alternate audio engine</span> named [kira][kira], *alongside* vanilla one.

It then does integrate seamlessly, creating the illusion that there's only one and unique audio environment.

Here is, once again in an overly simplified way, how it works:

![Audioware plugin](./assets/with-audioware.png)

---

But let's process to [next chapter](./HOWTO.md) to see how it can be used, and what it can currently do for you.

[^1]: *vanilla* describes everything belonging to the original game, as opposed to further modifications or *mods* made by the community.

[^2]: *natively* in the sense that tool, assets and game itself speaks the exact same language leading to seamless integration.

[^3]: reserved to a handful of professional.

[kira]: https://docs.rs/kira/latest/kira "kira crate"
