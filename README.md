
# transcribe-rust

This is an experimental project to capture live audio from a microphone and and transcribe it.

## Requirements

Tested on Ubuntu 22.04.

* AWS account and IAM role configured with transcribe policies. 

You may need to install the following dependencies:
```
sudo apt install librust-alsa-sys-dev
```

## Features
* Read a `.wav` file
* Capture audio from microphone (WIP)

## Notes

* If there's an issue with the transcription, try increasing sample rate.

## References

* [cpal](https://github.com/RustAudio/cpal)
* [Open issue to add Pulse Audio support in cpal](https://github.com/RustAudio/cpal/issues/259)
