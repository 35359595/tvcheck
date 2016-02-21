#tvcheck
Check and download your series from fs.to

For now working only with aria2c as downloader.

To add more then one link to series manualy edit `~/.tvcheck/list` and add link to .txt from fs.to in a new line.

Tested only on linux!

#HOWTO
run `tvcheck -h` for help menu of `tvcheck -v` for version info.

First launch will ask for link to `"Список серий"` from fs.to

To add new series with watched episoded: `tvcheck -add http://linkto/list?folder=0001` Warning: adding from bash with full link generates `&` to split it into two commands and causes malfunction.
Add without `&quality=webdl` and select quality by adding last parameter `sd` or `hd`

To add new series without watched episoded (didnt watch any episode yet): `tvcheck -new http://linkto/list?folder=0001` Same here: no `&quality=webdl` and `sd` or `hd` for quality selection.

#TODO
- [DONE]Gnome notificstions with libnotify; -Native RUST notification without libnotify!
- Automatic opening options;
- [Partly DONE]Parameters to manage series;
