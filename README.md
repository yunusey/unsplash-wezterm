# unsplash-wezterm

Use Unsplash's API to get fantastic wallpapers in WezTerm's background.

## Installation

First, you need to clone and build the project:
```bash
git clone https://github.com/yunusey/unsplash-wezterm.git
cd unsplash-wezterm && cargo build --release
```

Congrats, you are all set!

## Usage

When I started working on this project, I was trying to explore FFI (Foreign Function Interface). I thought [WezTerm](https://github.com/wez/wezterm) would let me use FFI with Rust since it is using Lua as its configuration language and there's a [builtin package](https://luajit.org/ext_ffi.html) in LuaJIT for this. However, as it turns out, it doesn't... for more info you can check [this issue](https://github.com/wez/wezterm/issues/2088). So, we need to run it using shell--which makes this project a bit ridiculous, because if what you're trying to do is to create an executable, you don't need FFI at all: just use Rust! But I like getting things fancy a little bit, so we're going to be calling `main.py` inside our root dir fetch and save image, then it will print out where it saved the file to, and we will use this file path to change the background image in WezTerm. This is how I did in my configuration:

```lua
local wezterm = require("wezterm")
local config = wezterm.config_builder()

---Your old config here

config.keys = {
	-- Your old keys here
	{
		key = 'n',
		mods = 'LEADER',
		action = wezterm.action_callback(function(window, pane)
			local path_to_py = '/path/to/unsplash-wezterm/main.py'
			local params = {
				api_key = 'YOUR-API-KEY',
				folder = '/path/to/where/you/want/to/save/images/to',
				collections = '',
				topics = '',
				username = '',
				query = 'winter',
				orientation = '',
				content_filter = '',
			}
			local command = os.getenv('SHELL') ..' -c \'python3 ' .. path_to_py
			for k, v in pairs(params) do
				command = command .. ' --' .. k .. '="' .. v .. '"'
			end
			command = command .. '\''

			wezterm.log_info("unsplash command: " .. command)
			local handle = io.popen(command)
			if handle == nil then
				return
			end
			---@type string
			local res = handle:read("*a")
			handle:close()

			wezterm.log_info("unsplash request: " .. res)
			-- trim new lines
			res = string.gsub(res, "^%s*(.-)%s*$", "%1")

			-- There was a problem with the request...
			if res == nil or res == 'ERROR' then
				return
			end

			local overrides = window:get_config_overrides() or {}
			overrides.window_background_image = res
			window:set_config_overrides(overrides)
		end)
	}
}
```

Please don't forget to change keys and mods as you wish. Also, you can do very exciting things with queries and collections, so don't just stick to one particular query (or do so!) :D. One last thing is that you don't have to specify anything other than `folder` and `api_key`. If you don't specify anything, then they won't take any effect in fetching images--pretty straight-forward! For the API key part, I recommend setting `UNSPLASH_API_KEY` variable in environment (please see PS below), and don't manually set key in your WezTerm config. It is up to you though!

_PS: Make sure that you put your `UNSPLASH_API_KEY` in your `.zshenv` or somewhere that you know WezTerm will source it. If not, the api key will not be set, and `main.py` will not be able to make the request. This is very important! To test if you have access to `UNSPLASH_API_KEY` in your config is to run `os.getenv("UNSPLASH_API_KEY")` and see if it is `nil` or not._

## Where else can I use this?

As you saw in the config above, `main.py` is independent from WezTerm. So, you can just call this:
```bash
python3 /path/to/unsplash-wezterm/main.py --folder="/path/to/where/you/want/to/save/images/to" --api_key="YOUR_API_KEY" --query="winter"
```

It will save the image to the folder you specified, and print out the result or if there's a problem in requesting the image (for example, if you exceeded the limit), it will print out `ERROR`.

## More about parameters

### Defaults
The program sets some defaults for particular parameters:
- `folder`: `CWD` by default
- `api_key`: `UNSPLASH_API_KEY` by default
- `orientation`: "landscape" by default

### How about `collections`, `topics`, `username`, `query`, and `content_filter`?
They are set as "" by default, and they are not passed to the API. You can set them as you wish. To learn more about them, I recommend checking out [this link](https://unsplash.com/documentation#get-a-random-photo). I don't know much about [Unsplash](https://unsplash.com), so the only parameter that I like to use is `query`, but the other parameters are supported.

## License

This project is licensed under the MIT license. So, feel free to use it for any purpose you want.

## Acknowledgements

- [WezTerm](https://github.com/wez/wezterm)
- [Unsplash](https://unsplash.com)

## Thanks
Thank you for reading, hope you like it! If you think that this project looks good, please consider leaving a ⭐ on GitHub.
