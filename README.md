◉ This is a browser extension for Arbitrary Protocols on top of Bitcoin (Bitcoin NFTs/Ordinals, 
BRC-20, & Bitcoin Identities/Usernames). It was built using 
[example extensions for Firefox](https://github.com/mdn/webextensions-examples) and 
https://github.com/neon64/chrome-native-messaging as a basis.

## Setup ##

To get this working, there's a little setup to do.

### Mac OS/Linux Setup ###

1. Build `arb-native-messaging-host` from source using `cargo build --release`.
2. Edit the "path" property of "arb_companion.json" to point to the location of the 
"arb-native-messaging-host" binary on your computer.
3. Place a copy of the `arb` binary in the same directory as the `arb-native-messaging-host` binary.
4. Copy "arb_companion.json" to the correct location on your computer. 
See [app manifest location ](https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Native_manifests#Manifest_location) 
to find the correct location for your OS.

### Windows Setup ###

1. Build `arb-native-messaging-host` from source using `cargo build --release`.
2. Edit the "path" property of "arb_companion.json" to point to the location of the 
"arb-native-messaging-host" binary on your computer. Note that you'll need to escape the Windows 
directory separator, like this: `"path": "C:\\Users\\arb-companion\\arb-native-messaging-host"`.
3. Place a copy of the `arb` binary in the same directory as the `arb-native-messaging-host` binary.
4. Add a registry key containing the path to "arb_companion.json" on your computer. 
See [app manifest location ](https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Native_manifests#Manifest_location) 
to find details of the registry key to add.

## Using the Extension ##

Then just install the browser extension as usual, by visiting about:debugging in Firefox, clicking 
"Load Temporary Add-on", and selecting the "manifest.json" file, or by visiting 
chrome://extensions/ in Chrome, toggling "Developer mode" to on, clicking "Load unpacked", and 
selecting the "manifest.json" file.

Note: Some things still need to be changed for this to run in Chrome, so only Firefox works for now.
