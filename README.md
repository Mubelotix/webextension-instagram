# webextension-instagram

This is an instagram bot, running in a webextension, running with wasm in Rust!  
This extension is intended to run in Firefox. Of course you can adapt the structure of the code for others browsers but it would be extremely boring because wasm is not supported at all in webextension for now.

## How to install

This version does not have many features for now, so the extension not published on <https://addons.mozilla.org/fr/firefox/>.  
Now, you must download this repository and use the [about:debugging](about:debugging) page.  
Select "This Firefox" and then click the "Load temporary addon" button.  
Then select the manifest.json file in the pkg folder you have downloaded.  
**"temporary addon" means that you must do this operation each time you start firefox.**  
Otherwise the extension will be deactivated.

## How to use

Just visit an instagram page.  
Don't try to use the "properties" button for now because it don't work.  
*Warning: You can do a lot of likes a day if you have more than 100 followers, but there is a limit (arround 250 a day for me). If you reach the limit, the warning message from instagram will not be detected for the bot (it will in the future). Ignoring this message is useless because likes are ignored and it may let instagram think your account is suspect. So you must verify manually from times to times (I recommend every 50 likes) that the action of giving a like does not open a window (because the bot switch to next post directly after liking, you can't see this window if the bot is running, the window is closed immediatly).* 

### How to launch a rust wasm file in a webextension content-script

That's very tricky. Because firefox does not support javascript module in webextension (content-script), you can't use wasm-bindgen in the same way than for a web page. However there is a way to bypass this limitation. Inject in the targeted page with javascript from content-script the javascript code needed to load and run wasm file wich require module. *Howerver*, the code run in the page and not in the extension and the permissions cannot be managed properly...