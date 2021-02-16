# tweeting-stop-light

A real stoplight that responds to Twitter mentions and hashtags using Rust.

To update the stoplight, tweet at [@TweetStopLight](https://twitter.com/TweetStopLight) using...<br>
🟢 #Green<br>
🟡 #Yellow<br>
🔴 #Red<br>

# Components

I used the following parts to construct this project:

## 1️⃣ Raspberry Pi model 3A+

![Rasbperry Pi](https://i.imgur.com/GjoWvFs.jpg)

A Raspberry Pi Model 3A+ with a mere 2GB of storage.  This is one of the weaker (and cheaper) Raspberry Pis available.  But I didn't need much computing power for this project since the Pi just needs to (1) run our Rust program on loop and (2) connect to the internet.

## 2️⃣ Relay Board

![Relay Board](https://i.imgur.com/ejlBtQE.jpg)

A board of 3 relays designed for the Raspberry Pi.  Since the Raspberry Pi is _DC powered_ and the stoplight is _AC powered_, we can't simply power the lights on the stoplight via the power from the GPIO pins.  Instead, these relays are here to convert from DC to AC and essentially act as on/off switches for the lights on the stoplight.

## 3️⃣ Stoplight

![Stoplight](https://i.imgur.com/gvF9zIL.jpg)

A classic Red/Amber/Green traffic light with 8" lenses, wired for a household 120V outlet.  I nabbed this bad-boy from Ebay for the small price of $145, plus $100 shipping and handling.  Ironically, this was the easiest component to get ahold of.  I just needed a stoplight that was (1) wired for regular 120V outlets and (2) did _not_ come installed with any traffic timers installed, since I would just be replacing this with the Raspberry Pi.

# Construction

The Raspbery Pi, relays and stoplight all come together as seen in the image below.  The Raspberry Pi runs our Rust program on loop, which controls what relay is open using the GPIO pins.  Each of the three lights on the stoplight are wired to pass through one of the relays.  This way the Raspberry Pi can control which light is on at any given time.

![Inner Workings](https://i.imgur.com/K0Zl1cB.jpg)

# Why Rust?

I decided to use Rust for this project purely as a challenge.  Rust is probably one of the most promising up-and-coming programming frameworks, and this was the perfect project to use as an introduction.

By using Rust, I was able to reap the benefit of a more optimized program.  This played into my decision to get the cheapest Raspberry Pi, since I knew I wouldn't need much computing power to run the executable that Rust outputs.

However, this whole project could have been completed in much less time by using Python instead.  Since Rust is still a new programming language, the community is not quite there yet.  At first, I struggled to simply find a package that could make a request to the Twitter API.

The community needs more time to cover all of the most common use cases.  But hopefully that time will be coming soon since the [Rust Foundation](https://foundation.rust-lang.org/posts/2021-02-08-hello-world/) has just been announced.