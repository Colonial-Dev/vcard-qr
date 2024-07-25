# vcard-qr
Generate basic vCard QR codes from your terminal. 
Stick them on your things so people can contact you if they get lost!

This little program is only vaguely serious;
I wrote it after hearing about Tile's [lost and found labels](https://www.tile.com/product/lost-and-found-labels) product, 
which will charge you *$15* for... some QR codes pointing to a web page with your information on it. 
Now, I'm not exactly sure what the total cost of a mass-produced sticker sheet and a few kilobytes of database storage is, 
but I'm going to make an educated guess and say its somewhere in the ballpark of a few pennies. 

"What a ripoff!" I thought. "And, come to think of it, can't QR codes contain vCards...?"

And thus `vcard-qr` was born.

## Features
- Interactively generate vCard QR codes.
- Free and open source.
- Works offline forever.

Or, expressed in a way my fellow Zoomers will have an easier time understanding:

![pls do not take this meme seriously](https://imgur.com/ONxH1DS.png)

## Usage
Just invoke `cargo run` or `vcard-qr` and answer the interactive prompts. 
Most information is optional; in accordance with the vCard spec, technically only a name is required, 
but you probably want to specify at least an email or a phone number.
If you want, you can also specify:

- A website.
- An address or addresses.
- A custom (multiline) note.

Once you've been thoroughly prompted, the program will render your vCard and QR Code to the disk.
By default it's formatted as a 1024x1024 SVG, but this is configurable - see below.

## Configuration
There are a few arguments you can change from the default to tweak the final output:
- `-o`/`--output-name` - the name of the output file, sans extension. Defaults to `vcard`.
- `-f`/`--format` - the output format of the QR code, either `png` or `svg`. Defaults to `png`.
- `-e`/`--error-correction` - how much error correction should be baked into the QR code - `low`, `medium`, `high`, or `max`. Higher EC levels will generate larger QR codes, but can increase the chance that the code will remain readable if it's damaged. Defaults to `low`.
- `-s`/`--size` - The height and width of the output image, in pixels. Defaults to 1024.
- `--from` - Reuse an existing vCard file instead of prompting.
