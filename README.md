#tvcheck
Check and download your series from fs.to

For now working only with aria2c as downloader.

To add more then one link to series manualy edit `~/.tvcheck/list` and add link to .txt from fs.to in a new line.

Tested only on linux!

#HOWTO
run `tvcheck -h` for help menu of `tvcheck -v` for version info.

First launch will ask for link to `"Список серий"` from fs.to

To add new series with watched episoded: `tvcheck -a or --add  with link in quotes "http://linkto/list?folder=0001&quality=webdl"`.

To add new series without watched episoded (didnt watch any episode yet): `tvcheck -n or --new  with link in quotes "http://linkto/list?folder=0001&quality=hdtv"`.

To remove series: `tvcheck -r or --remove` and enter the number of series to remove from shown list.

To do suppress notification showing use -s or --silent.

All parameters can be grouped. Like -as or -ns

#WHATSNEW

v.0.4.6

	-Option 'Silent\Quiet' added;

v.0.4.5

	-Fix for new function;

	-Bugfix for file creations;

	-Minor bug fixes;

v.0.4.0

	-Rework of adding function;

	-Rework of notification function: added episode name to title and Play button;

	-Added Play function to play downloaded episode after download;

	-Rework of treading the result of download - check for download complete.

	-Minor bug fixes;

v.0.3.9

	-Minor bug fixes;

v.0.3.8

	-Added correct argumen parse for various quality links;

	-Adding new and seen series now in separate functions;

	-Help and about now correctly displaying for all arguments;

v.0.3.7
	
	-Bug fixe for add function;

#TODO

- [DONE]Gnome notificstions with libnotify; -Native RUST notification without libnotify!

- [DONE]Automatic opening options (play button from notification - opens episode with VLC);

- [Partly DONE]Parameters to manage series (currently no need in other parameters);
