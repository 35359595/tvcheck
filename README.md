#tvcheck
Check and download your series from fs.to

For now working only with aria2c as downloader.

To add more then one link to series manualy edit ~/.tvcheck/list and add link to .txt from fs.to in a new line.

Tested only on linux!

#HOWTO
First launch will ask for link to "Список серий" from fs.to 

`To add new series with watched episoded: tvcheck -add http://linkto/list?folder=0001&quality=webdl 

To add new series without watched episoded (didnt watch any episode yet): tvcheck -new http://linkto/list?folder=0001&quality=webdl

#TODO
- Gnome notificstions with libnotify;
- Automatic opening options;
- Parameters to manage series (Partly done with -add and -new options);
