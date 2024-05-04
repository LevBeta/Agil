# Agil: High-Performance Trading Base [WIP FOR FIRST RELEASE]

# DO NOT USE IT YET, READ THIS TWEET ABOUT IT: https://x.com/levbeta/status/1786453984921694617

Agil is a high-performance trading base designed for high-frequency (HFT) and medium-frequency (MFT) trading. It prioritizes speed, modularity, and user-friendliness to excel in the fast-paced trading landscape. Key features include:

- **Performance**: Agil is engineered for lightning-fast execution, giving you a competitive edge in trading.
  
- **Modularity**: Its modular design allows for easy customization to suit specific trading strategies.
  
- **User Experience**: Agil offers an intuitive interface suitable for both seasoned traders and newcomers, ensuring quick comprehension and smooth operation.

Agil originated as a fork of [Barter-Data](https://github.com/barter-rs/barter-data-rs), with credit to the original [author](https://github.com/just-a-stream) and contributors. This led to the development of [Agil-Data](https://github.com/LevBeta/Agil/tree/master/crates/agil-data), a faster and more robust solution.

## Current TODO

Currently there is some parts of the code that already have a tag of "TODO:", so it's easier to find someparts

* **HIGH IMPORTANCE**: (Since this happend kek, the lib file and the ExchangeWsStream can be the first to be lookped up and rewritten)
* Look into the shit of [`TransformerSelector`] which should be deleted, it was some tests that i had made before
* move the fast-websockets parts into their respective and better locations.
* Clean placeholder code, unused deps, move things to their correct locations, add docs etc.

* **Mid/high importance**: Start working on a better solution/ rewrite some parts of the private feed implementation 
