# Slack Imposters

Pretend to be your coworkers on Slack! It’s easy as can be!

Simply clone, build, and then create a `impost.yaml` file in the cloned repo
root. (Yes, yes, these are not real-world instructions. This is a toy for me to
learn Rust and make something silly I like to do easier.)


## Example Config File

```yaml
channel: "default-channel-to-post-to",
boturl: "https://hooks.slack.com/services/bot-url"
bill:
  username: "bbt"
  avatar: "http://static2.quoteswave.com/wp-content/uploads/2012/09/Billy-Bob-Thornton-50x50.jpg"
```

## Usage

```sh
# post in the general channel
impost --as bill --channel general -t "Hello friends! :wave: I’m back from the astroid!"
# or even send a direct message to bill as bill.
impost --as bill --channel @bill -t "Hello friends! :wave: I’m back from the astroid!"
```

## MIT License

The MIT License (MIT)

Copyright (c) 2016 Dustan Kasten

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
