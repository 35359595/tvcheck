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

#WHATSNEW

v.0.3.8

	-Added correct argumen parse for various quality links;

	-Adding new and seen series now in separate functions;

	-Help and about now correctly displaying for all arguments;

v.0.3.7
	
	-Bug fixe for add function;

#TODO
- [DONE]Gnome notificstions with libnotify; -Native RUST notification without libnotify!
- Automatic opening options;
- [Partly DONE]Parameters to manage series;
