# YT Song Downloader
This is a command line tool that downloads youtube videos but only their audio track. The cli is made using clap and the actual downloading using rustube.

## How to use
Simply provide the url to the youtube video and optionally provide a file name using the `-o` or `--out` argument. If you want the file to be converted to the output format using ffmpeg you can use the `-c` argument. A call could look something like this:
`yt-song-downloader.exe https://www.youtube.com/watch?v=dQw4w9WgXcQ -o "song.mp3" -c`