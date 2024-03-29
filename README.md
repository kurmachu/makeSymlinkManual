# makeSymlinkManual
makeSymlinkManual is a small command line tool originally written in C#, now recreated in Rust, to help create folder symlinks manually. “Manually” is separate to specifying a path in pre-existing symlink commands as seemingly all of them rely on the destination being available in order to determine whether the link should be a file or folder. **makeSymlinkManual was created specifically to avoid this restriction, with the intended use being to create symlinks to folders that do not currently exist.** An example of this is a drive mounted only under one (non-admin) user, such as what Google Drive for Desktop does.
## Usage
makeSymlinkManual takes two paramaters, the first one is the location of the symlink you would like to create, the second is where you would like the symlink to go.
```powershell
mkSymlinkManual.exe "<from>" "<to>"
```
If you've compiled makeSymlinkManual from source, be sure to use `make_symlink_manual.exe` (or rename your copy to `mkSymlinkManual.exe`)
