cli-about = A supercharged archiving tool

cli-quiet-help = reduce output. Multiple occurences make output less informative
cli-quiet-long-help =
    Decrease the logging level. The default level is info.
    Each occurrance decreases the level from info to warn to error to nothing.
cli-verbose-help = explain what is beeing done. Multiple occurences make output more informative
cli-verbose-long-help =
    Increase the logging level. The default level is info.
    Each occurrance increases the level from info to debug to trace.

cli-info-about = Display info on an archive

cli-pack-about = Pack files and directories into an archive
cli-pack-format-help = Specify the compression format
cli-pack-compression-help = Specify the compresion level [possible values: auto, 0-9]
cli-pack-compression-long-help =
    Specify the compression level from 0..9.
    Default if no value specified or not set is auto. 
    auto means compression level is decided by the format

    Example:
    `-c` -> auto
    `-c 0` -> 0
cli-pack-compression-value-name = level
cli-pack-input-help = files and directories to pack
cli-pack-output-help = output file

cli-unpack-about = Unpack an archive