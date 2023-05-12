# murmur

murmur into your terminal and convert your speech to text using `OpenAI`'s
Whisper API.

Records a WAV file using the default input device and config until the user
indicates end of input.

The input data is recorded to "$`CARGO_MANIFEST_DIR/recorded.wav`".

### Installation

Note: if you're using [ALSA](https://www.alsa-project.org/wiki/Main_Page),
you may need to install the development files for `libasound2`,

```console
$ sudo apt install libasound2-dev
```

License: MIT
